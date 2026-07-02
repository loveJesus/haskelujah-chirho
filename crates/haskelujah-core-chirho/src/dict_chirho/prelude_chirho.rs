// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! Prelude function generation: all generate_*_prelude_chirho methods.

#![allow(unused_imports)]
use std::collections::HashMap;

use haskelujah_span_chirho::SpanChirho;
use haskelujah_typing_chirho::class_chirho::ClassEnvChirho;
use haskelujah_typing_chirho::ty_chirho::{TyChirho, TyVarChirho};

use super::{DictLayoutChirho, DictPassCtxChirho};
use crate::expr_chirho::{
    AltConChirho, BinderChirho, CoreAltChirho, CoreBindingChirho, CoreExprChirho, CoreIdChirho,
    CoreLitChirho, CoreModuleChirho, InlineAnnotationChirho,
};

impl DictPassCtxChirho {
    pub fn generate_prelude_bindings_chirho(&mut self) {
        let bool_ty_chirho = TyChirho::bool_chirho();

        // Helper to generate a Prelude binding with ID matching.
        // Uses resolve_or_fresh_id_chirho so the binding ID matches
        // any reference the desugarer may have already created.
        let prelude_fns_chirho: Vec<(
            &str,
            TyChirho,
            Box<dyn FnOnce(&mut Self) -> CoreExprChirho>,
        )> = vec![
            // not :: Bool -> Bool
            (
                "not",
                TyChirho::fun_chirho(bool_ty_chirho.clone(), bool_ty_chirho.clone()),
                Box::new(|ctx_chirho: &mut Self| {
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", TyChirho::bool_chirho());
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "not#".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                        }),
                    }
                }),
            ),
            // otherwise :: Bool  (otherwise = True)
            (
                "otherwise",
                bool_ty_chirho.clone(),
                Box::new(|_ctx_chirho: &mut Self| CoreExprChirho::ConAppChirho {
                    con_name_chirho: "True".to_string(),
                    args_chirho: vec![],
                }),
            ),
            // id :: a -> a
            (
                "id",
                {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    TyChirho::fun_chirho(a_chirho.clone(), a_chirho)
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }
                }),
            ),
            // fromList :: [a] -> [a]  (IsList identity for standard lists)
            (
                "fromList",
                {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    TyChirho::fun_chirho(a_chirho.clone(), a_chirho)
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }
                }),
            ),
            // toList :: [a] -> [a]  (IsList identity for standard lists)
            (
                "toList",
                {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    TyChirho::fun_chirho(a_chirho.clone(), a_chirho)
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }
                }),
            ),
            // const :: a -> b -> a
            (
                "const",
                {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let b_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
                    TyChirho::fun_chirho(a_chirho.clone(), TyChirho::fun_chirho(b_chirho, a_chirho))
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let b_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho);
                    let y_chirho = ctx_chirho.fresh_binder_chirho("y", b_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho,
                            body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                        }),
                    }
                }),
            ),
            // flip :: (a -> b -> c) -> b -> a -> c
            (
                "flip",
                {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let b_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
                    let c_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9992));
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            a_chirho.clone(),
                            TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                        ),
                        TyChirho::fun_chirho(b_chirho, TyChirho::fun_chirho(a_chirho, c_chirho)),
                    )
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
                    let b_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
                    let c_chirho =
                        TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9992));
                    let f_chirho = ctx_chirho.fresh_binder_chirho(
                        "f",
                        TyChirho::fun_chirho(
                            a_chirho.clone(),
                            TyChirho::fun_chirho(b_chirho.clone(), c_chirho),
                        ),
                    );
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", b_chirho);
                    let y_chirho = ctx_chirho.fresh_binder_chirho("y", a_chirho);
                    // flip f x y = f y x
                    CoreExprChirho::LamChirho {
                        binder_chirho: f_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: x_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::LamChirho {
                                binder_chirho: y_chirho.clone(),
                                body_chirho: Box::new(CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            f_chirho.id_chirho,
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            y_chirho.id_chirho,
                                        )),
                                    }),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                        x_chirho.id_chirho,
                                    )),
                                }),
                            }),
                        }),
                    }
                }),
            ),
            // subtract :: Int -> Int -> Int
            (
                "subtract",
                {
                    let int_chirho = TyChirho::int_chirho();
                    TyChirho::fun_chirho(
                        int_chirho.clone(),
                        TyChirho::fun_chirho(int_chirho.clone(), int_chirho),
                    )
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let int_chirho = TyChirho::int_chirho();
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", int_chirho.clone());
                    let y_chirho = ctx_chirho.fresh_binder_chirho("y", int_chirho);
                    CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "-#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                                ],
                            }),
                        }),
                    }
                }),
            ),
        ];

        for (name_chirho, ty_chirho, make_rhs_chirho) in prelude_fns_chirho {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let rhs_chirho = make_rhs_chirho(self);
            let binder_chirho = BinderChirho {
                id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Some interface stubs expose newtypes without feeding the local
        // newtype-erasure table, so give their constructor/accessor names
        // body-backed identity functions.
        for name_chirho in [
            "Identity",
            "runIdentity",
            "Sum",
            "getSum",
            "Product",
            "getProduct",
            "All",
            "getAll",
            "Any",
            "getAny",
        ] {
            let a_chirho = TyChirho::VarChirho(TyVarChirho(9980));
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho, x_chirho.ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // even :: Int -> Bool
        // even n = n `mod` 2 == 0
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let even_id_chirho = self.resolve_or_fresh_id_chirho("even");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());

            let mod_result_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "mod#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                ],
            };
            let eq_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    mod_result_chirho,
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(eq_zero_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: even_id_chirho,
                name_chirho: "even".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // odd :: Int -> Bool
        // odd n = n `mod` 2 /= 0
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let odd_id_chirho = self.resolve_or_fresh_id_chirho("odd");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());

            let mod_result_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "mod#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                ],
            };
            let ne_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "/=#".to_string(),
                args_chirho: vec![
                    mod_result_chirho,
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(ne_zero_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: odd_id_chirho,
                name_chirho: "odd".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // abs :: Int -> Int
        // abs n = if n < 0 then negate n else n
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let abs_id_chirho = self.resolve_or_fresh_id_chirho("abs");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());

            let lt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };
            let negated_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "negate#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
            };

            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(lt_zero_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: negated_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(body_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: abs_id_chirho,
                name_chirho: "abs".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho: rhs_chirho.clone(),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // Also generate $prim_Num_abs_Int for the instance dictionary
            let prim_abs_name_chirho = "$prim_Num_abs_Int";
            let prim_abs_id_chirho = self.resolve_or_fresh_id_chirho(prim_abs_name_chirho);
            let prim_abs_binder_chirho = BinderChirho {
                id_chirho: prim_abs_id_chirho,
                name_chirho: prim_abs_name_chirho.to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: prim_abs_binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // max :: Int -> Int -> Int
        // max a b = if a >= b then a else b
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let max_id_chirho = self.resolve_or_fresh_id_chirho("max");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());

            let ge_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">=#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                ],
            };

            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(ge_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(b_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: max_id_chirho,
                name_chirho: "max".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    int_ty_chirho.clone(),
                    TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // min :: Int -> Int -> Int
        // min a b = if a <= b then a else b
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let min_id_chirho = self.resolve_or_fresh_id_chirho("min");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());

            let le_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<=#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                ],
            };

            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(le_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(a_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(b_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: min_id_chirho,
                name_chirho: "min".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    int_ty_chirho.clone(),
                    TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // fst :: (a, b) -> a
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let fst_id_chirho = self.resolve_or_fresh_id_chirho("fst");
            let p_chirho = self.fresh_binder_chirho("p", pair_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("$w", pair_ty_chirho.clone()),
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho.clone(), y_chirho],
                    rhs_chirho: CoreExprChirho::VarChirho(x_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(body_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: fst_id_chirho,
                name_chirho: "fst".to_string(),
                ty_chirho: TyChirho::fun_chirho(pair_ty_chirho.clone(), a_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // snd :: (a, b) -> b
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let snd_id_chirho = self.resolve_or_fresh_id_chirho("snd");
            let p_chirho = self.fresh_binder_chirho("p", pair_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("$w", pair_ty_chirho.clone()),
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho, y_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(y_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(body_chirho),
            };

            let binder_chirho = BinderChirho {
                id_chirho: snd_id_chirho,
                name_chirho: "snd".to_string(),
                ty_chirho: TyChirho::fun_chirho(pair_ty_chirho, b_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // curry :: ((a, b) -> c) -> a -> b -> c
        // curry f a b = f (a, b)
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
            let c_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9992));
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let curry_id_chirho = self.resolve_or_fresh_id_chirho("curry");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(pair_ty_chirho.clone(), c_chirho.clone()),
            );
            let a_binder_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let b_binder_chirho = self.fresh_binder_chirho("b", b_chirho.clone());

            // f (a, b)
            let tuple_expr_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(a_binder_chirho.id_chirho),
                    CoreExprChirho::VarChirho(b_binder_chirho.id_chirho),
                ],
            };
            let app_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(tuple_expr_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: a_binder_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_binder_chirho,
                        body_chirho: Box::new(app_chirho),
                    }),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: curry_id_chirho,
                name_chirho: "curry".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(pair_ty_chirho.clone(), c_chirho.clone()),
                    TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                    ),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // uncurry :: (a -> b -> c) -> (a, b) -> c
        // uncurry f (a, b) = f a b
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
            let c_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9992));
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let uncurry_id_chirho = self.resolve_or_fresh_id_chirho("uncurry");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                ),
            );
            let p_chirho = self.fresh_binder_chirho("p", pair_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            // case p of { (x, y) -> f x y }
            let f_applied_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("$w", pair_ty_chirho.clone()),
                result_ty_chirho: c_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho, y_chirho],
                    rhs_chirho: f_applied_chirho,
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let binder_chirho = BinderChirho {
                id_chirho: uncurry_id_chirho,
                name_chirho: "uncurry".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        a_chirho,
                        TyChirho::fun_chirho(b_chirho, c_chirho.clone()),
                    ),
                    TyChirho::fun_chirho(pair_ty_chirho, c_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // enumFromTo :: Int -> Int -> [Int]
        // enumFromTo from to = if from > to then [] else from : enumFromTo (from+1) to
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let enum_id_chirho = self.resolve_or_fresh_id_chirho("enumFromTo");
            let from_chirho = self.fresh_binder_chirho("from", int_ty_chirho.clone());
            let to_chirho = self.fresh_binder_chirho("to", int_ty_chirho.clone());

            // from > to
            let cond_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(from_chirho.id_chirho),
                    CoreExprChirho::VarChirho(to_chirho.id_chirho),
                ],
            };

            // from + 1
            let from_plus_one_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(from_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            };

            // enumFromTo (from+1) to
            let recursive_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(enum_id_chirho)),
                    arg_chirho: Box::new(from_plus_one_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(to_chirho.id_chirho)),
            };

            // from : enumFromTo (from+1) to
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(from_chirho.id_chirho),
                    recursive_call_chirho,
                ],
            };

            // []
            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            // case (from > to) of { True -> []; _ -> from : ... }
            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cond_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: TyChirho::VarChirho(
                    haskelujah_typing_chirho::ty_chirho::TyVarChirho(9998),
                ),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: from_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: to_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            let list_ty_chirho = TyChirho::ListChirho(Box::new(int_ty_chirho.clone()));
            let enum_binder_chirho = BinderChirho {
                id_chirho: enum_id_chirho,
                name_chirho: "enumFromTo".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    int_ty_chirho.clone(),
                    TyChirho::fun_chirho(int_ty_chirho, list_ty_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: enum_binder_chirho,
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // take :: Int -> [a] -> [a]
        // take n xs = case n <=# 0 of { True -> []; False -> case xs of { [] -> []; x:rest -> x : take (n -# 1) rest } }
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_ty_chirho = TyChirho::string_chirho(); // placeholder type
            let take_id_chirho = self.resolve_or_fresh_id_chirho("take");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_ty_chirho.clone());
            let wild1_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$wild", list_ty_chirho.clone());

            // n -# 1
            let n_minus_1_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "-#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            };
            // take (n-1) rest
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(take_id_chirho)),
                    arg_chirho: Box::new(n_minus_1_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            // x : take (n-1) rest
            let cons_result_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };
            // Inner case: case xs of { [] -> []; x:rest -> x : take (n-1) rest }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild2_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: cons_result_chirho,
                    },
                ],
            };
            // Outer case: case n <=# 0 of { True -> []; False -> inner }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild1_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: take_id_chirho,
                name_chirho: "take".to_string(),
                ty_chirho: int_ty_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // drop :: Int -> [a] -> [a]
        // drop n xs = case n <=# 0 of { True -> xs; False -> case xs of { [] -> []; _:rest -> drop (n-1) rest } }
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_ty_chirho = TyChirho::string_chirho(); // placeholder type
            let drop_id_chirho = self.resolve_or_fresh_id_chirho("drop");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            let _x_chirho = self.fresh_binder_chirho("_x", int_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_ty_chirho.clone());
            let wild1_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$wild", list_ty_chirho.clone());

            // n -# 1
            let n_minus_1_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "-#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                ],
            };
            // drop (n-1) rest
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(drop_id_chirho)),
                    arg_chirho: Box::new(n_minus_1_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            // Inner case: case xs of { [] -> []; _:rest -> drop (n-1) rest }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild2_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![_x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };
            // Outer case: case n <=# 0 of { True -> xs; False -> inner }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild1_chirho,
                result_ty_chirho: list_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(xs_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: drop_id_chirho,
                name_chirho: "drop".to_string(),
                ty_chirho: int_ty_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // words :: String -> [String]
        // words s = wordsStr# s
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let words_id_chirho = self.resolve_or_fresh_id_chirho("words");
            let s_chirho = self.fresh_binder_chirho("s", str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "wordsStr#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: words_id_chirho,
                name_chirho: "words".to_string(),
                ty_chirho: TyChirho::fun_chirho(str_ty_chirho, list_str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // unwords :: [String] -> String
        // unwords xs = unwordsStr# xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let unwords_id_chirho = self.resolve_or_fresh_id_chirho("unwords");
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "unwordsStr#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: unwords_id_chirho,
                name_chirho: "unwords".to_string(),
                ty_chirho: TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // lines :: String -> [String]
        // lines s = lines# s
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let lines_id_chirho = self.resolve_or_fresh_id_chirho("lines");
            let s_chirho = self.fresh_binder_chirho("s", str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "lines#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: lines_id_chirho,
                name_chirho: "lines".to_string(),
                ty_chirho: TyChirho::fun_chirho(str_ty_chirho, list_str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // unlines :: [String] -> String
        // unlines xs = unlines# xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let unlines_id_chirho = self.resolve_or_fresh_id_chirho("unlines");
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "unlines#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: unlines_id_chirho,
                name_chirho: "unlines".to_string(),
                ty_chirho: TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // concat :: [[a]] -> [a]
        // concat [] = []; concat (xs:xss) = append xs (concat xss)
        {
            let a_chirho = TyChirho::VarChirho(TyVarChirho(9984));
            let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
            let list_list_a_chirho = TyChirho::ListChirho(Box::new(list_a_chirho.clone()));
            let concat_id_chirho = self.resolve_or_fresh_id_chirho("concat");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let xss_chirho = self.fresh_binder_chirho("xss", list_list_a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_list_a_chirho.clone());

            let concat_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(concat_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let append_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(concat_rec_chirho),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xss_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![xs_chirho, rest_chirho],
                        rhs_chirho: append_call_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xss_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: concat_id_chirho,
                name_chirho: "concat".to_string(),
                ty_chirho: TyChirho::fun_chirho(list_list_a_chirho, list_a_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // intercalate :: String -> [String] -> String
        // intercalate sep xs = intercalateStr# sep xs
        {
            let str_ty_chirho = TyChirho::string_chirho();
            let list_str_ty_chirho = TyChirho::ListChirho(Box::new(str_ty_chirho.clone()));
            let intercalate_id_chirho = self.resolve_or_fresh_id_chirho("intercalate");
            let sep_chirho = self.fresh_binder_chirho("sep", str_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_str_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "intercalateStr#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(sep_chirho.id_chirho),
                    CoreExprChirho::VarChirho(xs_chirho.id_chirho),
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: sep_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: intercalate_id_chirho,
                name_chirho: "intercalate".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    str_ty_chirho.clone(),
                    TyChirho::fun_chirho(list_str_ty_chirho, str_ty_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // toInteger :: Int -> Int  (identity)
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let to_integer_id_chirho = self.resolve_or_fresh_id_chirho("toInteger");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let binder_chirho = BinderChirho {
                id_chirho: to_integer_id_chirho,
                name_chirho: "toInteger".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // fromIntegral :: Int -> Double
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let double_ty_chirho = TyChirho::double_chirho();
            let from_integral_id_chirho = self.resolve_or_fresh_id_chirho("fromIntegral");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "fromIntegral#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: from_integral_id_chirho,
                name_chirho: "fromIntegral".to_string(),
                ty_chirho: TyChirho::fun_chirho(int_ty_chirho, double_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ceiling :: Double -> Int
        // floor :: Double -> Int
        // round :: Double -> Int
        // truncate :: Double -> Int
        for (name_chirho, primop_chirho) in [
            ("ceiling", "ceiling#"),
            ("floor", "floor#"),
            ("round", "round#"),
            ("truncate", "truncate#"),
        ] {
            let double_ty_chirho = TyChirho::double_chirho();
            let int_ty_chirho = TyChirho::int_chirho();
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", double_ty_chirho.clone());
            let body_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: primop_chirho.to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: fn_id_chirho,
                name_chirho: name_chirho.to_string(),
                ty_chirho: TyChirho::fun_chirho(double_ty_chirho, int_ty_chirho),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // isJust :: Maybe a -> Bool
        // isJust m = case m of { Nothing -> False; Just _ -> True }
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9900));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let is_just_id_chirho = self.resolve_or_fresh_id_chirho("isJust");
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: is_just_id_chirho,
                name_chirho: "isJust".to_string(),
                ty_chirho: TyChirho::fun_chirho(maybe_a_chirho.clone(), TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // isNothing :: Maybe a -> Bool
        // isNothing m = case m of { Nothing -> True; Just _ -> False }
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9901));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let is_nothing_id_chirho = self.resolve_or_fresh_id_chirho("isNothing");
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };
            let binder_chirho = BinderChirho {
                id_chirho: is_nothing_id_chirho,
                name_chirho: "isNothing".to_string(),
                ty_chirho: TyChirho::fun_chirho(maybe_a_chirho, TyChirho::bool_chirho()),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // fromMaybe :: a -> Maybe a -> a
        // fromMaybe def m = case m of { Nothing -> def; Just x -> x }
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9902));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let from_maybe_id_chirho = self.resolve_or_fresh_id_chirho("fromMaybe");
            let def_chirho = self.fresh_binder_chirho("def", a_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(def_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: from_maybe_id_chirho,
                name_chirho: "fromMaybe".to_string(),
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::fun_chirho(maybe_a_chirho, a_chirho),
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // maybe :: b -> (a -> b) -> Maybe a -> b
        // maybe def f m = case m of { Nothing -> def; Just x -> f x }
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9903));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9904));
            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );
            let maybe_id_chirho = self.resolve_or_fresh_id_chirho("maybe");
            let def_chirho = self.fresh_binder_chirho("def", b_chirho.clone());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
            );
            let m_chirho = self.fresh_binder_chirho("m", maybe_a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(def_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: maybe_id_chirho,
                name_chirho: "maybe".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        b_chirho.clone(),
                        TyChirho::fun_chirho(a_chirho, b_chirho.clone()),
                        maybe_a_chirho,
                    ],
                    b_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // either :: (a -> c) -> (b -> c) -> Either a b -> c
        // either f g e = case e of { Left x -> f x; Right y -> g y }
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9905));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9906));
            let c_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9907));
            let either_ab_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Either".to_string())),
                    Box::new(a_chirho.clone()),
                )),
                Box::new(b_chirho.clone()),
            );
            let either_id_chirho = self.resolve_or_fresh_id_chirho("either");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), c_chirho.clone()),
            );
            let g_chirho = self.fresh_binder_chirho(
                "g",
                TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
            );
            let e_chirho = self.fresh_binder_chirho("e", either_ab_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: c_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                        binders_chirho: vec![y_chirho.clone()],
                        rhs_chirho: CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(g_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: g_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: e_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            let binder_chirho = BinderChirho {
                id_chirho: either_id_chirho,
                name_chirho: "either".to_string(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(a_chirho, c_chirho.clone()),
                        TyChirho::fun_chirho(b_chirho, c_chirho.clone()),
                        either_ab_chirho,
                    ],
                    c_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho,
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── min/max :: Int -> Int -> Int (via <=# primop) ─────────────
        {
            let int_chirho = TyChirho::ConChirho("Int".to_string());
            for (fn_name_chirho, pick_first_chirho) in [("min", true), ("max", false)] {
                let x_id_chirho = self.fresh_id_chirho("x");
                let y_id_chirho = self.fresh_id_chirho("y");
                let fn_id_chirho = self.resolve_or_fresh_id_chirho(fn_name_chirho);
                let wild_id_chirho = self.fresh_id_chirho("$wild");
                let x_chirho = BinderChirho {
                    id_chirho: x_id_chirho,
                    name_chirho: "x".to_string(),
                    ty_chirho: int_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                let y_chirho = BinderChirho {
                    id_chirho: y_id_chirho,
                    name_chirho: "y".to_string(),
                    ty_chirho: int_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                let wild_chirho = BinderChirho {
                    id_chirho: wild_id_chirho,
                    name_chirho: "$wild".to_string(),
                    ty_chirho: TyChirho::bool_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                // case x <=# y of True -> <first>; False -> <second>
                let (true_rhs_chirho, false_rhs_chirho) = if pick_first_chirho {
                    // min: True → x, False → y
                    (
                        CoreExprChirho::VarChirho(x_id_chirho),
                        CoreExprChirho::VarChirho(y_id_chirho),
                    )
                } else {
                    // max: True → y, False → x
                    (
                        CoreExprChirho::VarChirho(y_id_chirho),
                        CoreExprChirho::VarChirho(x_id_chirho),
                    )
                };
                let body_chirho = CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "<=#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_id_chirho),
                            CoreExprChirho::VarChirho(y_id_chirho),
                        ],
                    }),
                    bind_chirho: wild_chirho,
                    result_ty_chirho: int_chirho.clone(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: true_rhs_chirho,
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("False".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: false_rhs_chirho,
                        },
                    ],
                };
                let rhs_chirho = CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: y_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                };
                let binder_chirho = BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: fn_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), int_chirho.clone()],
                        int_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                };
                self.generated_bindings_chirho.push(CoreBindingChirho {
                    binder_chirho,
                    rhs_chirho,
                    is_rec_chirho: false,
                    inline_chirho: InlineAnnotationChirho::NoneChirho,
                });
            }
        }

        // ── Higher-order list functions ──────────────────────────────────
        self.generate_list_prelude_chirho();
    }

    /// Generate higher-order list Prelude functions: map, filter, foldr, foldl,
    /// head, tail, null, length, reverse, concatMap, zip, zipWith, sum, product.
    fn generate_list_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };

        // map :: (a -> b) -> [a] -> [b]
        // map f [] = []; map f (x:xs) = f x : map f xs
        {
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let map_f_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(map_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![f_h_chirho, map_f_t_chirho],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: map_id_chirho,
                    name_chirho: "map".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_b_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // filter :: (a -> Bool) -> [a] -> [a]
        // filter p [] = []; filter p (x:xs) = if p x then x : filter p xs else filter p xs
        {
            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let cw_chirho = self.fresh_binder_chirho("$cw", TyChirho::bool_chirho());

            let filter_p_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    filter_p_t_chirho.clone(),
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let if_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: cw_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: filter_p_t_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: if_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: filter_id_chirho,
                    name_chirho: "filter".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // foldr :: (a -> b -> b) -> b -> [a] -> b
        // foldr f z [] = z; foldr f z (x:xs) = f x (foldr f z xs)
        {
            let foldr_id_chirho = self.resolve_or_fresh_id_chirho("foldr");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone()),
                ),
            );
            let z_chirho = self.fresh_binder_chirho("z", b_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let foldr_f_z_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldr_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let f_h_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(foldr_f_z_t_chirho),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(z_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: f_h_rec_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: foldr_id_chirho,
                    name_chirho: "foldr".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            a_chirho.clone(),
                            TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone()),
                        ),
                        TyChirho::fun_chirho(
                            b_chirho.clone(),
                            TyChirho::fun_chirho(list_a_chirho.clone(), b_chirho.clone()),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // foldl :: (b -> a -> b) -> b -> [a] -> b
        // foldl f z [] = z; foldl f z (x:xs) = foldl f (f z x) xs
        {
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(
                    b_chirho.clone(),
                    TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                ),
            );
            let z_chirho = self.fresh_binder_chirho("z", b_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_z_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let foldl_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(f_z_h_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(z_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: foldl_rec_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: foldl_id_chirho,
                    name_chirho: "foldl".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            b_chirho.clone(),
                            TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                        ),
                        TyChirho::fun_chirho(
                            b_chirho.clone(),
                            TyChirho::fun_chirho(list_a_chirho.clone(), b_chirho.clone()),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // head :: [a] -> a
        // head (x:_) = x
        {
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho.clone(), t_chirho],
                    rhs_chirho: CoreExprChirho::VarChirho(h_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: head_id_chirho,
                    name_chirho: "head".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // tail :: [a] -> [a]
        // tail (_:xs) = xs
        {
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho, t_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(t_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tail_id_chirho,
                    name_chirho: "tail".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // null :: [a] -> Bool
        // null [] = True; null _ = False
        {
            let null_id_chirho = self.resolve_or_fresh_id_chirho("null");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: null_id_chirho,
                    name_chirho: "null".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // length :: [a] -> Int
        // length [] = 0; length (_:xs) = 1 + length xs
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let length_id_chirho = self.resolve_or_fresh_id_chirho("length");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let length_t_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(length_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let one_plus_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    length_t_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: one_plus_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: length_id_chirho,
                    name_chirho: "length".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), int_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // reverse :: [a] -> [a]
        // reverse [] = []; reverse (x:xs) = reverse xs ++ [x]
        // (implemented as foldl with flip cons)
        {
            let reverse_id_chirho = self.resolve_or_fresh_id_chirho("reverse");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let _h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let _t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let _w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let _acc_chirho = self.fresh_binder_chirho("acc", list_a_chirho.clone());

            // reverse xs = go [] xs where go acc [] = acc; go acc (h:t) = go (h:acc) t
            let go_id_chirho = {
                let id_chirho = CoreIdChirho(self.next_id_chirho);
                self.next_id_chirho += 1;
                self.names_chirho.insert(id_chirho, "$rev_go".to_string());
                id_chirho
            };

            let go_acc_chirho = self.fresh_binder_chirho("acc", list_a_chirho.clone());
            let go_ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let go_h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let go_t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let go_w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let h_cons_acc_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(go_h_chirho.id_chirho),
                    CoreExprChirho::VarChirho(go_acc_chirho.id_chirho),
                ],
            };
            let go_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                    arg_chirho: Box::new(h_cons_acc_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(go_t_chirho.id_chirho)),
            };

            let go_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(go_ys_chirho.id_chirho)),
                bind_chirho: go_w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(go_acc_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![go_h_chirho, go_t_chirho],
                        rhs_chirho: go_rec_chirho,
                    },
                ],
            };

            let go_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: go_acc_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: go_ys_chirho,
                    body_chirho: Box::new(go_body_chirho),
                }),
            };

            let go_binder_chirho = BinderChirho {
                id_chirho: go_id_chirho,
                name_chirho: "$rev_go".to_string(),
                ty_chirho: list_a_chirho.clone(),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            };

            let rev_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: vec![(go_binder_chirho, go_lam_chirho)],
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(go_id_chirho)),
                        arg_chirho: Box::new(nil_chirho()),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(rev_body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: reverse_id_chirho,
                    name_chirho: "reverse".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // zip :: [a] -> [b] -> [(a,b)]
        // zip (a:as) (b:bs) = (a,b) : zip as bs; zip _ _ = []
        {
            let pair_ty_chirho = TyChirho::TupleChirho(vec![a_chirho.clone(), b_chirho.clone()]);
            let list_pair_chirho = TyChirho::ListChirho(Box::new(pair_ty_chirho.clone()));
            let zip_id_chirho = self.resolve_or_fresh_id_chirho("zip");
            let as_chirho = self.fresh_binder_chirho("as", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let ah_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let at_chirho = self.fresh_binder_chirho("at", list_a_chirho.clone());
            let bh_chirho = self.fresh_binder_chirho("b", b_chirho.clone());
            let bt_chirho = self.fresh_binder_chirho("bt", list_b_chirho.clone());
            let wa_chirho = self.fresh_binder_chirho("$wa", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", list_b_chirho.clone());

            let pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(ah_chirho.id_chirho),
                    CoreExprChirho::VarChirho(bh_chirho.id_chirho),
                ],
            };
            let zip_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(zip_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(at_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bt_chirho.id_chirho)),
            };
            let cons_pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![pair_chirho, zip_rec_chirho],
            };

            // Inner case: case bs of { [] -> []; (b:bt) -> (a,b) : zip at bt }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_pair_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![bh_chirho, bt_chirho],
                        rhs_chirho: cons_pair_chirho,
                    },
                ],
            };

            // Outer case: case as of { [] -> []; (a:at) -> <inner_case> }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                bind_chirho: wa_chirho,
                result_ty_chirho: list_pair_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![ah_chirho, at_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: as_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: bs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zip_id_chirho,
                    name_chirho: "zip".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_b_chirho.clone(), list_pair_chirho),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // zipWith :: (a -> b -> c) -> [a] -> [b] -> [c]
        {
            let c_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9992));
            let list_c_chirho = TyChirho::ListChirho(Box::new(c_chirho.clone()));
            let zipw_id_chirho = self.resolve_or_fresh_id_chirho("zipWith");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                ),
            );
            let as_chirho = self.fresh_binder_chirho("as", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let ah_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let at_chirho = self.fresh_binder_chirho("at", list_a_chirho.clone());
            let bh_chirho = self.fresh_binder_chirho("b", b_chirho.clone());
            let bt_chirho = self.fresh_binder_chirho("bt", list_b_chirho.clone());
            let wa_chirho = self.fresh_binder_chirho("$wa", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", list_b_chirho.clone());

            let f_a_b_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(ah_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bh_chirho.id_chirho)),
            };
            let zipw_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(zipw_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(at_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bt_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![f_a_b_chirho, zipw_rec_chirho],
            };

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_c_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![bh_chirho, bt_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                bind_chirho: wa_chirho,
                result_ty_chirho: list_c_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![ah_chirho, at_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: as_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: bs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zipw_id_chirho,
                    name_chirho: "zipWith".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            a_chirho.clone(),
                            TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
                        ),
                        TyChirho::fun_chirho(
                            list_a_chirho.clone(),
                            TyChirho::fun_chirho(list_b_chirho.clone(), list_c_chirho),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // append :: [a] -> [a] -> [a]
        // append [] ys = ys; append (x:xs) ys = x : append xs ys
        {
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let append_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    append_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: append_id_chirho,
                    name_chirho: "append".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // any :: (a -> Bool) -> [a] -> Bool
        // any p [] = False; any p (x:xs) = case p x of { True -> True; False -> any p xs }
        {
            let any_id_chirho = self.resolve_or_fresh_id_chirho("any");
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let cw_chirho = self.fresh_binder_chirho("$cw", TyChirho::bool_chirho());

            let any_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(any_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: cw_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: any_rec_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: any_id_chirho,
                    name_chirho: "any".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // all :: (a -> Bool) -> [a] -> Bool
        // all p [] = True; all p (x:xs) = case p x of { True -> all p xs; False -> False }
        {
            let all_id_chirho = self.resolve_or_fresh_id_chirho("all");
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let cw_chirho = self.fresh_binder_chirho("$cw", TyChirho::bool_chirho());

            let all_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(all_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: cw_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: all_rec_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: all_id_chirho,
                    name_chirho: "all".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // sum :: [Int] -> Int  (Int-specialized)
        // sum [] = 0; sum (x:xs) = x +# sum xs
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_int_chirho = TyChirho::ListChirho(Box::new(int_ty_chirho.clone()));
            let sum_id_chirho = self.resolve_or_fresh_id_chirho("sum");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", int_ty_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_int_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_int_chirho.clone());

            let sum_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(sum_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let add_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "+#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    sum_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: add_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: sum_id_chirho,
                    name_chirho: "sum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho, int_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // product :: [Int] -> Int  (Int-specialized)
        // product [] = 1; product (x:xs) = x *# product xs
        {
            let int_ty_chirho = TyChirho::int_chirho();
            let list_int_chirho = TyChirho::ListChirho(Box::new(int_ty_chirho.clone()));
            let product_id_chirho = self.resolve_or_fresh_id_chirho("product");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", int_ty_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_int_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_int_chirho.clone());

            let product_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(product_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let mul_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "*#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    product_rec_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: mul_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: product_id_chirho,
                    name_chirho: "product".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho, int_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // concatMap :: (a -> [b]) -> [a] -> [b]
        // concatMap f [] = []; concatMap f (x:xs) = append (f x) (concatMap f xs)
        {
            let concatmap_id_chirho = self.resolve_or_fresh_id_chirho("concatMap");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), list_b_chirho.clone()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let concatmap_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(concatmap_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let append_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(f_h_chirho),
                }),
                arg_chirho: Box::new(concatmap_rec_chirho),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: append_call_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: concatmap_id_chirho,
                    name_chirho: "concatMap".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), list_b_chirho.clone()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_b_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // last :: [a] -> a
        // last (x:xs) = case xs of { [] -> x; _ -> last xs }
        {
            let last_id_chirho = self.resolve_or_fresh_id_chirho("last");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", list_a_chirho.clone());
            let h2_chirho = self.fresh_binder_chirho("h2", a_chirho.clone());
            let t2_chirho = self.fresh_binder_chirho("t2", list_a_chirho.clone());

            let last_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(last_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h2_chirho, t2_chirho],
                        rhs_chirho: last_rec_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho, t_chirho],
                    rhs_chirho: inner_case_chirho,
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: last_id_chirho,
                    name_chirho: "last".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // init :: [a] -> [a]
        // init [x] = []; init (x:xs) = x : init xs
        {
            let init_id_chirho = self.resolve_or_fresh_id_chirho("init");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", list_a_chirho.clone());
            let h2_chirho = self.fresh_binder_chirho("h2", a_chirho.clone());
            let t2_chirho = self.fresh_binder_chirho("t2", list_a_chirho.clone());

            let init_rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(init_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    init_rec_chirho,
                ],
            };
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h2_chirho, t2_chirho],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho(":".to_string()),
                    binders_chirho: vec![h_chirho, t_chirho],
                    rhs_chirho: inner_case_chirho,
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: init_id_chirho,
                    name_chirho: "init".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Additional list functions: takeWhile, dropWhile, iterate, lookup, unzip, scanl ──
        self.generate_extra_list_prelude_chirho();

        // ── Additional list functions: nub, nubBy, group, groupBy, tails, inits ──
        self.generate_list_extra_prelude_chirho();

        // ── Ord-based list functions: elem, notElem, minimum, maximum, sort ──
        self.generate_ord_prelude_chirho();

        // ── Enum / Bounded prim bindings for instance dictionaries ──
        self.generate_enum_bounded_prelude_chirho();

        // ── Integral prim bindings for instance dictionaries ──
        self.generate_integral_prelude_chirho();

        // ── Data.Char functions ──
        self.generate_char_prelude_chirho();

        // ── Floating prim bindings ──
        self.generate_floating_prelude_chirho();

        // ── Functor / Applicative / Monad prim bindings ──
        self.generate_functor_monad_prelude_chirho();

        // ── IO control flow ──
        self.generate_io_control_prelude_chirho();

        // ── Data.Map (BST-based) ──
        self.generate_map_prelude_chirho();

        // ── Data.Map extended operations ──
        self.generate_map_extended_chirho();

        // ── Data.Map String-keyed operations ──
        self.generate_map_str_prelude_chirho();

        // ── Data.Set (BST-based) ──
        self.generate_set_prelude_chirho();

        // ── Data.Maybe extras ──
        self.generate_maybe_prelude_chirho();

        // ── Data.IORef ──
        self.generate_ioref_prelude_chirho();

        // ── Control.Monad.ST ──
        self.generate_st_prelude_chirho();

        // ── Semigroup / Monoid ──
        self.generate_semigroup_monoid_prelude_chirho();

        // ── Higher-order list functions ──
        self.generate_higher_order_list_prelude_chirho();

        // ── Monad transformer infrastructure (MaybeT / StateT) ──
        self.generate_transformer_prelude_chirho();

        // ── Monad transformer evaluation (bind/return/get/put/modify) ──
        self.generate_transformer_monad_prelude_chirho();

        // ── Additional utility functions (repeat, cycle, group, transpose, fix, etc.) ──
        self.generate_utility_prelude_chirho();

        // ── Lazy arithmetic sequence functions (enumFrom, enumFromThen, enumFromTo, enumFromThenTo) ──
        self.generate_enum_sequence_prelude_chirho();

        // ── NFData / deepseq / evaluate / force ──
        self.generate_nfdata_prelude_chirho();
    }

    /// Generate Core IR bindings for monad transformer infrastructure.
    ///
    /// MaybeT is implemented as a newtype wrapping `Maybe a`.  At the Core IR
    /// level we represent it as a data constructor `MaybeT` with a single field
    /// and a selector `runMaybeT`.  StateT is similar: constructor `StateT`
    /// wrapping a state function `s -> (a, s)` with selector `runStateT`.
    ///
    /// Because the compiler's newtype erasure turns single-field constructors
    /// into identity at runtime, these bindings delegate directly through the
    /// `Maybe` / tuple machinery that already exists, making the resulting
    /// evaluation straightforward.
    fn generate_transformer_prelude_chirho(&mut self) {
        use haskelujah_typing_chirho::ty_chirho::TyVarChirho;

        let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let ty_s_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let ty_r_chirho = TyChirho::VarChirho(TyVarChirho(9992));
        let ty_e_chirho = TyChirho::VarChirho(TyVarChirho(9993));
        let ty_w_chirho = TyChirho::VarChirho(TyVarChirho(9994));
        let maybe_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(a_chirho.clone()),
        );
        // s -> (a, s)
        let state_fn_ty_chirho = TyChirho::fun_chirho(
            ty_s_chirho.clone(),
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("(,)".to_string())),
                    Box::new(a_chirho.clone()),
                )),
                Box::new(ty_s_chirho.clone()),
            ),
        );
        // StateT s a
        let state_t_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(ty_s_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        // ReaderT r a
        let reader_t_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(ty_r_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        // Either e a
        let either_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(ty_e_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        // ExceptT e a
        let except_t_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(ty_e_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        // (a, w)
        let pair_a_w_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("(,)".to_string())),
                Box::new(a_chirho.clone()),
            )),
            Box::new(ty_w_chirho.clone()),
        );
        // WriterT w a
        let writer_t_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(ty_w_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        // MaybeT a
        let maybe_t_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
            Box::new(a_chirho.clone()),
        );

        // ── MaybeT constructor: MaybeT :: Maybe a -> MaybeT a ──
        // Implementation: identity wrapper (newtype erasure makes this trivial).
        // At runtime the `MaybeT` tag is emitted as a single-field constructor.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("MaybeT");
            let inner_chirho = self.fresh_binder_chirho("inner", maybe_a_chirho.clone());
            // MaybeT inner = Con "MaybeT" [inner]
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: inner_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "MaybeT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(inner_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "MaybeT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        maybe_a_chirho.clone(),
                        maybe_t_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runMaybeT :: MaybeT a -> Maybe a ──
        // Implementation: case dispatch extracts the single field.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runMaybeT");
            let arg_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_s", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("inner", maybe_a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: maybe_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("MaybeT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_chirho.id_chirho),
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: arg_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runMaybeT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        maybe_t_ty_chirho.clone(),
                        maybe_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── StateT constructor: StateT :: (s -> (a, s)) -> StateT s a ──
        // Implementation: wraps a state-transition function.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("StateT");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "StateT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(f_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "StateT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        state_fn_ty_chirho.clone(),
                        state_t_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runStateT :: StateT s a -> s -> (a, s) ──
        // Implementation: case dispatch extracts the state function, then applies it.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runStateT");
            let t_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sr", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("f", a_chirho.clone());

            // case t of { StateT f -> f s }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("StateT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(field_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                    },
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: t_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runStateT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        state_t_ty_chirho.clone(),
                        TyChirho::fun_chirho(
                            ty_s_chirho.clone(),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::ConChirho("(,)".to_string())),
                                    Box::new(a_chirho.clone()),
                                )),
                                Box::new(ty_s_chirho.clone()),
                            ),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── ReaderT constructor: ReaderT :: (r -> a) -> ReaderT r a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("ReaderT");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "ReaderT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(f_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "ReaderT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(ty_r_chirho.clone(), a_chirho.clone()),
                        reader_t_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runReaderT :: ReaderT r a -> r -> a ──
        // Implementation: case dispatch extracts the reader function, then applies it.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runReaderT");
            let t_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sr", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("f", a_chirho.clone());

            // case t of { ReaderT f -> f r }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ReaderT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(field_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                    },
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: t_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: r_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runReaderT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        reader_t_ty_chirho.clone(),
                        TyChirho::fun_chirho(ty_r_chirho.clone(), a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── ExceptT constructor: ExceptT :: Either e a -> ExceptT e a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("ExceptT");
            let inner_chirho = self.fresh_binder_chirho("inner", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: inner_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "ExceptT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(inner_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "ExceptT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        either_ty_chirho.clone(),
                        except_t_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runExceptT :: ExceptT e a -> Either e a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runExceptT");
            let arg_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_s", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("inner", a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ExceptT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_chirho.id_chirho),
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: arg_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runExceptT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        except_t_ty_chirho.clone(),
                        either_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── WriterT constructor: WriterT :: (a, w) -> WriterT w a ──
        // Wraps a (value, log) pair.
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("WriterT");
            let inner_chirho = self.fresh_binder_chirho("inner", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: inner_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "WriterT".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(inner_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "WriterT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        pair_a_w_ty_chirho.clone(),
                        writer_t_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runWriterT :: WriterT w a -> (a, w) ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runWriterT");
            let arg_chirho = self.fresh_binder_chirho("t", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_s", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("inner", a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(arg_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("WriterT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_chirho.id_chirho),
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: arg_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runWriterT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        writer_t_ty_chirho.clone(),
                        pair_a_w_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Core IR bindings for monad transformer operations.
    ///
    /// StateT operations:
    ///   get       :: StateT s s           — get = StateT (\s -> (s, s))
    ///   put       :: s -> StateT s ()     — put s' = StateT (\_ -> ((), s'))
    ///   modify    :: (s -> s) -> StateT s () — modify f = StateT (\s -> ((), f s))
    ///   runState  :: StateT s a -> s -> (a, s)  — synonym for runStateT
    ///   evalState :: StateT s a -> s -> a — evalState m s = fst (runStateT m s)
    ///   execState :: StateT s a -> s -> s — execState m s = snd (runStateT m s)
    ///
    /// MaybeT operations:
    ///   returnMaybeT :: a -> MaybeT a     — returnMaybeT x = MaybeT (Just x)
    ///   bindMaybeT   :: MaybeT a -> (a -> MaybeT b) -> MaybeT b
    fn generate_transformer_monad_prelude_chirho(&mut self) {
        use haskelujah_typing_chirho::ty_chirho::TyVarChirho;
        // Generic type used as placeholder for binder types (runtime-erased)
        let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        // Proper type variables for accurate type annotations
        let ty_a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let ty_s_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let ty_r_chirho = TyChirho::VarChirho(TyVarChirho(9992));
        let ty_e_chirho = TyChirho::VarChirho(TyVarChirho(9993));
        let ty_b_chirho = TyChirho::VarChirho(TyVarChirho(9995));
        // StateT s a
        let state_t_s_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(ty_s_chirho.clone()),
            )),
            Box::new(ty_a_chirho.clone()),
        );
        // StateT s ()
        let state_t_s_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(ty_s_chirho.clone()),
            )),
            Box::new(TyChirho::ConChirho("()".to_string())),
        );
        // StateT s s
        let state_t_s_s_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(ty_s_chirho.clone()),
            )),
            Box::new(ty_s_chirho.clone()),
        );
        // StateT s b
        let state_t_s_b_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("StateT".to_string())),
                Box::new(ty_s_chirho.clone()),
            )),
            Box::new(ty_b_chirho.clone()),
        );
        // (a, s) — result tuple
        let pair_a_s_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("(,)".to_string())),
                Box::new(ty_a_chirho.clone()),
            )),
            Box::new(ty_s_chirho.clone()),
        );
        // ReaderT r a
        let reader_t_r_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                Box::new(ty_r_chirho.clone()),
            )),
            Box::new(ty_a_chirho.clone()),
        );
        // ExceptT e a
        let except_t_e_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                Box::new(ty_e_chirho.clone()),
            )),
            Box::new(ty_a_chirho.clone()),
        );

        // ── get :: StateT s s ──
        // get = StateT (\s -> (s, s))
        {
            let get_id_chirho = self.resolve_or_fresh_id_chirho("get");
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            // \s -> (s, s)
            let state_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "$tuple2".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(s_chirho.id_chirho),
                        CoreExprChirho::VarChirho(s_chirho.id_chirho),
                    ],
                }),
            };
            // StateT (\s -> (s, s))
            let rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "StateT".to_string(),
                args_chirho: vec![state_fn_chirho],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: get_id_chirho,
                    name_chirho: "get".to_string(),
                    ty_chirho: state_t_s_s_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── put :: s -> StateT s () ──
        // put s' = StateT (\_ -> ((), s'))
        {
            let put_id_chirho = self.resolve_or_fresh_id_chirho("put");
            let s_prime_chirho = self.fresh_binder_chirho("s'", a_chirho.clone());
            let wildcard_chirho = self.fresh_binder_chirho("_", a_chirho.clone());
            // \_ -> ((), s')
            let inner_chirho = CoreExprChirho::LamChirho {
                binder_chirho: wildcard_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "$tuple2".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: "()".to_string(),
                            args_chirho: vec![],
                        },
                        CoreExprChirho::VarChirho(s_prime_chirho.id_chirho),
                    ],
                }),
            };
            // \s' -> StateT (\_ -> ((), s'))
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_prime_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "StateT".to_string(),
                    args_chirho: vec![inner_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: put_id_chirho,
                    name_chirho: "put".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        ty_s_chirho.clone(),
                        state_t_s_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── modify :: (s -> s) -> StateT s () ──
        // modify f = StateT (\s -> ((), f s))
        {
            let modify_id_chirho = self.resolve_or_fresh_id_chirho("modify");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            // \s -> ((), f s)
            let inner_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "$tuple2".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: "()".to_string(),
                            args_chirho: vec![],
                        },
                        CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                        },
                    ],
                }),
            };
            // \f -> StateT (\s -> ((), f s))
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "StateT".to_string(),
                    args_chirho: vec![inner_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: modify_id_chirho,
                    name_chirho: "modify".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(ty_s_chirho.clone(), ty_s_chirho.clone()),
                        state_t_s_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── evalState :: StateT s a -> s -> a ──
        // evalState m s = fst (runStateT m s)
        {
            let eval_state_id_chirho = self.resolve_or_fresh_id_chirho("evalState");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            let run_state_id_chirho = self.resolve_or_fresh_id_chirho("runStateT");
            let scr_chirho = self.fresh_binder_chirho("_res", a_chirho.clone());
            let fst_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let snd_chirho = self.fresh_binder_chirho("_s", a_chirho.clone());

            // case (runStateT m s) of { (a, _) -> a }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(run_state_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                }),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![fst_chirho.clone(), snd_chirho],
                    rhs_chirho: CoreExprChirho::VarChirho(fst_chirho.id_chirho),
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: eval_state_id_chirho,
                    name_chirho: "evalState".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        state_t_s_a_chirho.clone(),
                        TyChirho::fun_chirho(ty_s_chirho.clone(), ty_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── execState :: StateT s a -> s -> s ──
        // execState m s = snd (runStateT m s)
        {
            let exec_state_id_chirho = self.resolve_or_fresh_id_chirho("execState");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            let run_state_id_chirho = self.resolve_or_fresh_id_chirho("runStateT");
            let scr_chirho = self.fresh_binder_chirho("_res", a_chirho.clone());
            let fst_chirho = self.fresh_binder_chirho("_a", a_chirho.clone());
            let snd_chirho = self.fresh_binder_chirho("s2", a_chirho.clone());

            // case (runStateT m s) of { (_, s2) -> s2 }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(run_state_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                }),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![fst_chirho, snd_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(snd_chirho.id_chirho),
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: exec_state_id_chirho,
                    name_chirho: "execState".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        state_t_s_a_chirho.clone(),
                        TyChirho::fun_chirho(ty_s_chirho.clone(), ty_s_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── bindStateT :: StateT s a -> (a -> StateT s b) -> StateT s b ──
        // bindStateT m k = StateT (\s -> case runStateT m s of { (a, s') -> runStateT (k a) s' })
        {
            let bind_st_id_chirho = self.resolve_or_fresh_id_chirho("bindStateT");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());
            let run_state_id_chirho = self.resolve_or_fresh_id_chirho("runStateT");
            let scr_chirho = self.fresh_binder_chirho("_res", a_chirho.clone());
            let a_bind_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let s_prime_chirho = self.fresh_binder_chirho("s'", a_chirho.clone());

            // runStateT (k a) s'
            let cont_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(run_state_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(a_bind_chirho.id_chirho)),
                    }),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(s_prime_chirho.id_chirho)),
            };

            // case runStateT m s of { (a, s') -> runStateT (k a) s' }
            let case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(run_state_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
                }),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![a_bind_chirho, s_prime_chirho],
                    rhs_chirho: cont_chirho,
                }],
            };

            // StateT (\s -> case ...)
            let state_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho,
                body_chirho: Box::new(case_chirho),
            };
            let wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "StateT".to_string(),
                args_chirho: vec![state_fn_chirho],
            };

            // \m k -> StateT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho,
                    body_chirho: Box::new(wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: bind_st_id_chirho,
                    name_chirho: "bindStateT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        state_t_s_a_chirho.clone(),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(a_chirho.clone(), state_t_s_b_chirho.clone()),
                            state_t_s_b_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── returnStateT :: a -> StateT s a ──
        // returnStateT x = StateT (\s -> (x, s))
        {
            let ret_st_id_chirho = self.resolve_or_fresh_id_chirho("returnStateT");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", a_chirho.clone());

            let state_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "$tuple2".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::VarChirho(s_chirho.id_chirho),
                    ],
                }),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "StateT".to_string(),
                    args_chirho: vec![state_fn_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ret_st_id_chirho,
                    name_chirho: "returnStateT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), state_t_s_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runState :: StateT s a -> s -> (a, s)  (alias for runStateT) ──
        {
            let run_state_id_chirho = self.resolve_or_fresh_id_chirho("runState");
            let run_state_t_id_chirho = self.resolve_or_fresh_id_chirho("runStateT");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: run_state_id_chirho,
                    name_chirho: "runState".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        state_t_s_a_chirho.clone(),
                        TyChirho::fun_chirho(ty_s_chirho.clone(), pair_a_s_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(run_state_t_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // =====================================================================
        // ReaderT operations
        // =====================================================================

        // ── ask :: ReaderT r r ──
        // ask = ReaderT (\r -> r)   (identity function wrapped in ReaderT)
        {
            let ask_id_chirho = self.resolve_or_fresh_id_chirho("ask");
            let r_chirho = self.fresh_binder_chirho("r", a_chirho.clone());
            // \r -> r
            let reader_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            // ReaderT (\r -> r)
            let rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "ReaderT".to_string(),
                args_chirho: vec![reader_fn_chirho],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ask_id_chirho,
                    name_chirho: "ask".to_string(),
                    ty_chirho: TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                            Box::new(ty_r_chirho.clone()),
                        )),
                        Box::new(ty_r_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── local :: (r -> r) -> ReaderT r a -> ReaderT r a ──
        // local f m = ReaderT (\r -> runReaderT m (f r))
        {
            let local_id_chirho = self.resolve_or_fresh_id_chirho("local");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sl", a_chirho.clone());
            let field_chirho = self.fresh_binder_chirho("g", a_chirho.clone());

            // f r
            let f_r_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            // case m of { ReaderT g -> g (f r) }
            let inner_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ReaderT".to_string()),
                    binders_chirho: vec![field_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(field_chirho.id_chirho)),
                        arg_chirho: Box::new(f_r_chirho),
                    },
                }],
            };
            // ReaderT (\r -> case m of { ReaderT g -> g (f r) })
            let reader_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(inner_body_chirho),
            };
            let reader_wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "ReaderT".to_string(),
                args_chirho: vec![reader_fn_chirho],
            };
            // \f -> \m -> ReaderT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho,
                    body_chirho: Box::new(reader_wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: local_id_chirho,
                    name_chirho: "local".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(ty_r_chirho.clone(), ty_r_chirho.clone()),
                        TyChirho::fun_chirho(
                            reader_t_r_a_chirho.clone(),
                            reader_t_r_a_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── bindReaderT :: ReaderT r a -> (a -> ReaderT r b) -> ReaderT r b ──
        // bindReaderT m k = ReaderT (\r -> let a = runReaderT m r in runReaderT (k a) r)
        {
            let bind_id_chirho = self.resolve_or_fresh_id_chirho("bindReaderT");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("r", a_chirho.clone());
            let scr_m_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let field_m_chirho = self.fresh_binder_chirho("fm", a_chirho.clone());
            let a_val_chirho = self.fresh_binder_chirho("a_val", a_chirho.clone());
            let scr_k_chirho = self.fresh_binder_chirho("_sk", a_chirho.clone());
            let field_k_chirho = self.fresh_binder_chirho("fk", a_chirho.clone());

            // case m of { ReaderT fm -> fm r }  (this is runReaderT m r)
            let run_m_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_m_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ReaderT".to_string()),
                    binders_chirho: vec![field_m_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(field_m_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                    },
                }],
            };

            // k a_val  (produces ReaderT r b)
            let k_a_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(a_val_chirho.id_chirho)),
            };

            // case (k a_val) of { ReaderT fk -> fk r }  (this is runReaderT (k a) r)
            let run_k_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(k_a_chirho),
                bind_chirho: scr_k_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ReaderT".to_string()),
                    binders_chirho: vec![field_k_chirho.clone()],
                    rhs_chirho: CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(field_k_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                    },
                }],
            };

            // let a_val = runReaderT m r in runReaderT (k a_val) r
            let let_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(a_val_chirho, run_m_chirho)],
                body_chirho: Box::new(run_k_chirho),
            };

            // ReaderT (\r -> let a_val = ... in ...)
            let reader_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(let_body_chirho),
            };
            let reader_wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "ReaderT".to_string(),
                args_chirho: vec![reader_fn_chirho],
            };
            // \m -> \k -> ReaderT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho,
                    body_chirho: Box::new(reader_wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: bind_id_chirho,
                    name_chirho: "bindReaderT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        reader_t_r_a_chirho.clone(),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(
                                ty_a_chirho.clone(),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                                        Box::new(ty_r_chirho.clone()),
                                    )),
                                    Box::new(ty_b_chirho.clone()),
                                ),
                            ),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::ConChirho("ReaderT".to_string())),
                                    Box::new(ty_r_chirho.clone()),
                                )),
                                Box::new(ty_b_chirho.clone()),
                            ),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── returnReaderT :: a -> ReaderT r a ──
        // returnReaderT x = ReaderT (\_ -> x)
        {
            let ret_id_chirho = self.resolve_or_fresh_id_chirho("returnReaderT");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let r_chirho = self.fresh_binder_chirho("_r", a_chirho.clone());
            // \_ -> x
            let reader_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "ReaderT".to_string(),
                    args_chirho: vec![reader_fn_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ret_id_chirho,
                    name_chirho: "returnReaderT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), reader_t_r_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runReader :: ReaderT r a -> r -> a  (alias for runReaderT) ──
        {
            let run_reader_id_chirho = self.resolve_or_fresh_id_chirho("runReader");
            let run_reader_t_id_chirho = self.resolve_or_fresh_id_chirho("runReaderT");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: run_reader_id_chirho,
                    name_chirho: "runReader".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        reader_t_r_a_chirho.clone(),
                        TyChirho::fun_chirho(ty_r_chirho.clone(), ty_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(run_reader_t_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // =====================================================================
        // ExceptT operations
        // =====================================================================

        // ── throwE :: e -> ExceptT e a ──
        // throwE e = ExceptT (Left e)
        {
            let throw_id_chirho = self.resolve_or_fresh_id_chirho("throwE");
            let e_chirho = self.fresh_binder_chirho("e", a_chirho.clone());
            let left_val_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Left".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(e_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "ExceptT".to_string(),
                    args_chirho: vec![left_val_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: throw_id_chirho,
                    name_chirho: "throwE".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        ty_e_chirho.clone(),
                        except_t_e_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── returnExceptT :: a -> ExceptT e a ──
        // returnExceptT x = ExceptT (Right x)
        {
            let ret_id_chirho = self.resolve_or_fresh_id_chirho("returnExceptT");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let right_val_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Right".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "ExceptT".to_string(),
                    args_chirho: vec![right_val_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ret_id_chirho,
                    name_chirho: "returnExceptT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), except_t_e_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── bindExceptT :: ExceptT e a -> (a -> ExceptT e b) -> ExceptT e b ──
        // bindExceptT m k = ExceptT (case runExceptT m of
        //   Left e  -> Left e
        //   Right a -> runExceptT (k a))
        {
            let bind_id_chirho = self.resolve_or_fresh_id_chirho("bindExceptT");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let scr_m_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let field_m_chirho = self.fresh_binder_chirho("inner_m", a_chirho.clone());
            let scr_either_chirho = self.fresh_binder_chirho("_se", a_chirho.clone());
            let err_chirho = self.fresh_binder_chirho("err", a_chirho.clone());
            let val_chirho = self.fresh_binder_chirho("val", a_chirho.clone());
            let scr_k_chirho = self.fresh_binder_chirho("_sk", a_chirho.clone());
            let field_k_chirho = self.fresh_binder_chirho("inner_k", a_chirho.clone());

            // case m of { ExceptT inner_m -> inner_m }  (runExceptT m)
            let run_m_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_m_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ExceptT".to_string()),
                    binders_chirho: vec![field_m_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_m_chirho.id_chirho),
                }],
            };

            // k val  (produces ExceptT e b)
            let k_val_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(val_chirho.id_chirho)),
            };

            // case (k val) of { ExceptT inner_k -> inner_k }  (runExceptT (k val))
            let run_k_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(k_val_chirho),
                bind_chirho: scr_k_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ExceptT".to_string()),
                    binders_chirho: vec![field_k_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_k_chirho.id_chirho),
                }],
            };

            // case (runExceptT m) of
            //   Left err  -> Left err
            //   Right val -> runExceptT (k val)
            let either_dispatch_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(run_m_chirho),
                bind_chirho: scr_either_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                        binders_chirho: vec![err_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Left".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(err_chirho.id_chirho)],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                        binders_chirho: vec![val_chirho],
                        rhs_chirho: run_k_chirho,
                    },
                ],
            };

            // ExceptT (case ...)
            let except_wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "ExceptT".to_string(),
                args_chirho: vec![either_dispatch_chirho],
            };

            // \m -> \k -> ExceptT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho,
                    body_chirho: Box::new(except_wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: bind_id_chirho,
                    name_chirho: "bindExceptT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        except_t_e_a_chirho.clone(),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(
                                a_chirho.clone(),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                                        Box::new(ty_e_chirho.clone()),
                                    )),
                                    Box::new(ty_b_chirho.clone()),
                                ),
                            ),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                                    Box::new(ty_e_chirho.clone()),
                                )),
                                Box::new(ty_b_chirho.clone()),
                            ),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── catchE :: ExceptT e a -> (e -> ExceptT e a) -> ExceptT e a ──
        // catchE m handler = ExceptT (case runExceptT m of
        //   Left e  -> runExceptT (handler e)
        //   Right a -> Right a)
        {
            let catch_id_chirho = self.resolve_or_fresh_id_chirho("catchE");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let handler_chirho = self.fresh_binder_chirho("handler", a_chirho.clone());
            let scr_m_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let field_m_chirho = self.fresh_binder_chirho("inner_m", a_chirho.clone());
            let scr_either_chirho = self.fresh_binder_chirho("_se", a_chirho.clone());
            let err_chirho = self.fresh_binder_chirho("err", a_chirho.clone());
            let val_chirho = self.fresh_binder_chirho("val", a_chirho.clone());
            let scr_h_chirho = self.fresh_binder_chirho("_sh", a_chirho.clone());
            let field_h_chirho = self.fresh_binder_chirho("inner_h", a_chirho.clone());

            // case m of { ExceptT inner_m -> inner_m }
            let run_m_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_m_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ExceptT".to_string()),
                    binders_chirho: vec![field_m_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_m_chirho.id_chirho),
                }],
            };

            // handler err  (produces ExceptT e a)
            let handler_err_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(handler_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(err_chirho.id_chirho)),
            };

            // case (handler err) of { ExceptT inner_h -> inner_h }
            let run_handler_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(handler_err_chirho),
                bind_chirho: scr_h_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("ExceptT".to_string()),
                    binders_chirho: vec![field_h_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_h_chirho.id_chirho),
                }],
            };

            // case (runExceptT m) of
            //   Left err  -> runExceptT (handler err)
            //   Right val -> Right val
            let either_dispatch_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(run_m_chirho),
                bind_chirho: scr_either_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                        binders_chirho: vec![err_chirho],
                        rhs_chirho: run_handler_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                        binders_chirho: vec![val_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Right".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(val_chirho.id_chirho)],
                        },
                    },
                ],
            };

            // ExceptT (case ...)
            let except_wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "ExceptT".to_string(),
                args_chirho: vec![either_dispatch_chirho],
            };

            // \m -> \handler -> ExceptT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: handler_chirho,
                    body_chirho: Box::new(except_wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: catch_id_chirho,
                    name_chirho: "catchE".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        except_t_e_a_chirho.clone(),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(ty_e_chirho.clone(), except_t_e_a_chirho.clone()),
                            except_t_e_a_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // =====================================================================
        // MaybeT operations
        // =====================================================================

        // ── returnMaybeT :: a -> MaybeT a ──
        // returnMaybeT x = MaybeT (Just x)
        {
            let ret_id_chirho = self.resolve_or_fresh_id_chirho("returnMaybeT");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let just_x_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Just".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "MaybeT".to_string(),
                    args_chirho: vec![just_x_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ret_id_chirho,
                    name_chirho: "returnMaybeT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
                            Box::new(a_chirho.clone()),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── bindMaybeT :: MaybeT a -> (a -> MaybeT b) -> MaybeT b ──
        // bindMaybeT m k = MaybeT (case runMaybeT m of
        //   Nothing -> Nothing
        //   Just a  -> runMaybeT (k a))
        {
            let bind_id_chirho = self.resolve_or_fresh_id_chirho("bindMaybeT");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let scr_m_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let field_m_chirho = self.fresh_binder_chirho("inner_m", a_chirho.clone());
            let scr_maybe_chirho = self.fresh_binder_chirho("_smb", a_chirho.clone());
            let val_chirho = self.fresh_binder_chirho("val", a_chirho.clone());
            let scr_k_chirho = self.fresh_binder_chirho("_sk", a_chirho.clone());
            let field_k_chirho = self.fresh_binder_chirho("inner_k", a_chirho.clone());

            // case m of { MaybeT inner_m -> inner_m }  (runMaybeT m)
            let run_m_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_m_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("MaybeT".to_string()),
                    binders_chirho: vec![field_m_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_m_chirho.id_chirho),
                }],
            };

            // k val  (produces MaybeT b)
            let k_val_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(val_chirho.id_chirho)),
            };

            // case (k val) of { MaybeT inner_k -> inner_k }  (runMaybeT (k val))
            let run_k_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(k_val_chirho),
                bind_chirho: scr_k_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("MaybeT".to_string()),
                    binders_chirho: vec![field_k_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_k_chirho.id_chirho),
                }],
            };

            // case (runMaybeT m) of
            //   Nothing -> Nothing
            //   Just val -> runMaybeT (k val)
            let maybe_dispatch_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(run_m_chirho),
                bind_chirho: scr_maybe_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Nothing".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![val_chirho],
                        rhs_chirho: run_k_chirho,
                    },
                ],
            };

            // MaybeT (case ...)
            let maybe_wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "MaybeT".to_string(),
                args_chirho: vec![maybe_dispatch_chirho],
            };

            // \m -> \k -> MaybeT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho,
                    body_chirho: Box::new(maybe_wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: bind_id_chirho,
                    name_chirho: "bindMaybeT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("MaybeT".to_string())),
                            Box::new(a_chirho.clone()),
                        ),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(
                                ty_a_chirho.clone(),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::ConChirho("MaybeT".to_string())),
                                    Box::new(ty_b_chirho.clone()),
                                ),
                            ),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ConChirho("MaybeT".to_string())),
                                Box::new(ty_b_chirho.clone()),
                            ),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // =====================================================================
        // WriterT operations
        // =====================================================================

        // ── tell :: w -> WriterT w () ──
        // tell w = WriterT ((), w)
        {
            let tell_id_chirho = self.resolve_or_fresh_id_chirho("tell");
            let w_chirho = self.fresh_binder_chirho("w", a_chirho.clone());
            let pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "()".to_string(),
                        args_chirho: vec![],
                    },
                    CoreExprChirho::VarChirho(w_chirho.id_chirho),
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: w_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "WriterT".to_string(),
                    args_chirho: vec![pair_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tell_id_chirho,
                    name_chirho: "tell".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── returnWriterT :: a -> WriterT w a ──
        // returnWriterT x = WriterT (x, [])
        // (Using [] as mempty for the common [w] monoid case)
        {
            let ret_id_chirho = self.resolve_or_fresh_id_chirho("returnWriterT");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(),
                        args_chirho: vec![],
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "WriterT".to_string(),
                    args_chirho: vec![pair_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ret_id_chirho,
                    name_chirho: "returnWriterT".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── bindWriterT :: WriterT w a -> (a -> WriterT w b) -> WriterT w b ──
        // bindWriterT m k = WriterT (case runWriterT m of
        //   (a, w1) -> case runWriterT (k a) of
        //     (b, w2) -> (b, w1 ++ w2))
        {
            let bind_id_chirho = self.resolve_or_fresh_id_chirho("bindWriterT");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let scr_m_chirho = self.fresh_binder_chirho("_swm", a_chirho.clone());
            let field_m_chirho = self.fresh_binder_chirho("inner_m", a_chirho.clone());
            let scr_pair1_chirho = self.fresh_binder_chirho("_sp1", a_chirho.clone());
            let a_val_chirho = self.fresh_binder_chirho("a_val", a_chirho.clone());
            let w1_chirho = self.fresh_binder_chirho("w1", a_chirho.clone());
            let scr_k_chirho = self.fresh_binder_chirho("_swk", a_chirho.clone());
            let field_k_chirho = self.fresh_binder_chirho("inner_k", a_chirho.clone());
            let scr_pair2_chirho = self.fresh_binder_chirho("_sp2", a_chirho.clone());
            let b_val_chirho = self.fresh_binder_chirho("b_val", a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("w2", a_chirho.clone());
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");

            // case m of { WriterT inner_m -> inner_m }  (runWriterT m)
            let run_m_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_m_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("WriterT".to_string()),
                    binders_chirho: vec![field_m_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_m_chirho.id_chirho),
                }],
            };

            // k a_val  (produces WriterT w b)
            let k_a_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(a_val_chirho.id_chirho)),
            };

            // case (k a_val) of { WriterT inner_k -> inner_k }  (runWriterT (k a_val))
            let run_k_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(k_a_chirho),
                bind_chirho: scr_k_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("WriterT".to_string()),
                    binders_chirho: vec![field_k_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_k_chirho.id_chirho),
                }],
            };

            // w1 ++ w2
            let append_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(w1_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(w2_chirho.id_chirho)),
            };

            // (b_val, w1 ++ w2)
            let result_pair_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(b_val_chirho.id_chirho),
                    append_chirho,
                ],
            };

            // case (runWriterT (k a_val)) of { (b_val, w2) -> (b_val, w1 ++ w2) }
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(run_k_chirho),
                bind_chirho: scr_pair2_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![b_val_chirho, w2_chirho],
                    rhs_chirho: result_pair_chirho,
                }],
            };

            // case (runWriterT m) of { (a_val, w1) -> case ... }
            let outer_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(run_m_chirho),
                bind_chirho: scr_pair1_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![a_val_chirho, w1_chirho],
                    rhs_chirho: inner_case_chirho,
                }],
            };

            // WriterT (case ...)
            let writer_wrapped_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "WriterT".to_string(),
                args_chirho: vec![outer_case_chirho],
            };

            // \m -> \k -> WriterT (...)
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho,
                    body_chirho: Box::new(writer_wrapped_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: bind_id_chirho,
                    name_chirho: "bindWriterT".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runWriter :: WriterT w a -> (a, w)  (alias for runWriterT) ──
        {
            let run_writer_id_chirho = self.resolve_or_fresh_id_chirho("runWriter");
            let run_writer_t_id_chirho = self.resolve_or_fresh_id_chirho("runWriterT");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: run_writer_id_chirho,
                    name_chirho: "runWriter".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(run_writer_t_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── execWriterT :: WriterT w a -> w ──
        // execWriterT m = snd (runWriterT m)
        {
            let exec_id_chirho = self.resolve_or_fresh_id_chirho("execWriterT");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let scr_m_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let field_m_chirho = self.fresh_binder_chirho("inner_m", a_chirho.clone());
            let scr_pair_chirho = self.fresh_binder_chirho("_sp", a_chirho.clone());
            let fst_chirho = self.fresh_binder_chirho("_a", a_chirho.clone());
            let snd_chirho = self.fresh_binder_chirho("w", a_chirho.clone());

            // case m of { WriterT inner_m -> inner_m }
            let run_m_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_m_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("WriterT".to_string()),
                    binders_chirho: vec![field_m_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(field_m_chirho.id_chirho),
                }],
            };

            // case runWriterT m of { (_, w) -> w }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(run_m_chirho),
                bind_chirho: scr_pair_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![fst_chirho, snd_chirho.clone()],
                    rhs_chirho: CoreExprChirho::VarChirho(snd_chirho.id_chirho),
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: exec_id_chirho,
                    name_chirho: "execWriterT".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── execWriter :: WriterT w a -> w  (alias for execWriterT) ──
        {
            let exec_writer_id_chirho = self.resolve_or_fresh_id_chirho("execWriter");
            let exec_writer_t_id_chirho = self.resolve_or_fresh_id_chirho("execWriterT");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: exec_writer_id_chirho,
                    name_chirho: "execWriter".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(exec_writer_t_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate additional list functions: nub (Int), zip3, zipWith3, intersperse,
    /// isPrefixOf (Int), isSuffixOf (Int), tails, inits, and cycle.
    fn generate_list_extra_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
        let c_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9992));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));
        let _list_c_chirho = TyChirho::ListChirho(Box::new(c_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };
        let cons_chirho = |hd: CoreExprChirho, tl: CoreExprChirho| CoreExprChirho::ConAppChirho {
            con_name_chirho: ":".to_string(),
            args_chirho: vec![hd, tl],
        };

        // nub :: [Int] -> [Int]
        // nub [] = []
        // nub (x:xs) = x : nub (filter (\y -> not (y ==# x)) xs)
        // Uses elem-like recursive filtering
        {
            let nub_id_chirho = self.resolve_or_fresh_id_chirho("nub");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_ns", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());

            // helper: nubHelper seen [] = []
            //         nubHelper seen (x:xs) = if elem x seen then nubHelper seen xs
            //                                 else x : nubHelper (x:seen) xs
            // Simpler: inline recursive approach
            // nub [] = []
            // nub (x:xs) = x : nub (filter_ne x xs)
            // where filter_ne removes all elements == x

            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");

            // \y -> not (y ==# x)  — predicate to keep elements != h
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let eq_scr_chirho = self.fresh_binder_chirho("_eq", TyChirho::bool_chirho());

            let eq_check_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                ],
            };

            // not (y ==# x): case (y ==# x) of { True -> False; False -> True }
            let not_eq_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_check_chirho),
                bind_chirho: eq_scr_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            let pred_chirho = CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(not_eq_chirho),
            };

            // filter pred t
            let filtered_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(pred_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // nub (filter pred t)
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(nub_id_chirho)),
                arg_chirho: Box::new(filtered_chirho),
            };

            // h : nub (filter pred t)
            let cons_result_chirho =
                cons_chirho(CoreExprChirho::VarChirho(h_chirho.id_chirho), rec_chirho);

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_result_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: nub_id_chirho,
                    name_chirho: "nub".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // zip3 :: [a] -> [b] -> [c] -> [(a,b,c)]
        // zip3 [] _ _ = []
        // zip3 _ [] _ = []
        // zip3 _ _ [] = []
        // zip3 (a:as) (b:bs) (c:cs) = (a,b,c) : zip3 as bs cs
        {
            let zip3_id_chirho = self.resolve_or_fresh_id_chirho("zip3");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_b_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());
            let scr1_chirho = self.fresh_binder_chirho("_z1", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_z2", list_b_chirho.clone());
            let scr3_chirho = self.fresh_binder_chirho("_z3", list_a_chirho.clone());
            let xh_chirho = self.fresh_binder_chirho("xh", a_chirho.clone());
            let xt_chirho = self.fresh_binder_chirho("xt", list_a_chirho.clone());
            let yh_chirho = self.fresh_binder_chirho("yh", b_chirho.clone());
            let yt_chirho = self.fresh_binder_chirho("yt", list_b_chirho.clone());
            let zh_chirho = self.fresh_binder_chirho("zh", c_chirho.clone());
            let zt_chirho = self.fresh_binder_chirho("zt", list_a_chirho.clone());

            // (xh, yh, zh) tuple
            let tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(xh_chirho.id_chirho),
                    CoreExprChirho::VarChirho(yh_chirho.id_chirho),
                    CoreExprChirho::VarChirho(zh_chirho.id_chirho),
                ],
            };

            // zip3 xt yt zt
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(zip3_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(xt_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(yt_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(zt_chirho.id_chirho)),
            };

            let inner_cons_chirho = cons_chirho(tuple_chirho, rec_chirho);

            // case zs of { [] -> []; (z:zs) -> (x,y,z) : zip3 xt yt zt }
            let case_z_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(zs_chirho.id_chirho)),
                bind_chirho: scr3_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![zh_chirho, zt_chirho],
                        rhs_chirho: inner_cons_chirho,
                    },
                ],
            };

            // case ys of { [] -> []; (y:ys) -> case zs ... }
            let case_y_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![yh_chirho, yt_chirho],
                        rhs_chirho: case_z_chirho,
                    },
                ],
            };

            // case xs of { [] -> []; (x:xs) -> case ys ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr1_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![xh_chirho, xt_chirho],
                        rhs_chirho: case_y_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: zs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zip3_id_chirho,
                    name_chirho: "zip3".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // intersperse :: a -> [a] -> [a]
        // intersperse _ [] = []
        // intersperse _ [x] = [x]
        // intersperse sep (x:xs) = x : sep : intersperse sep xs
        {
            let isp_id_chirho = self.resolve_or_fresh_id_chirho("intersperse");
            let sep_chirho = self.fresh_binder_chirho("sep", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_is", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_is2", list_a_chirho.clone());

            // intersperse sep t
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(isp_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(sep_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // h : sep : intersperse sep t
            let cons_with_sep_chirho = cons_chirho(
                CoreExprChirho::VarChirho(h_chirho.id_chirho),
                cons_chirho(CoreExprChirho::VarChirho(sep_chirho.id_chirho), rec_chirho),
            );

            // case t of { [] -> [h]; _ -> h : sep : intersperse sep t }
            let h2_chirho = self.fresh_binder_chirho("_h2", a_chirho.clone());
            let t2_chirho = self.fresh_binder_chirho("_t2", list_a_chirho.clone());

            let check_tail_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho(
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            nil_chirho(),
                        ),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h2_chirho, t2_chirho],
                        rhs_chirho: cons_with_sep_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: check_tail_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: sep_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: isp_id_chirho,
                    name_chirho: "intersperse".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // isPrefixOf :: [Int] -> [Int] -> Bool
        // isPrefixOf [] _ = True
        // isPrefixOf _ [] = False
        // isPrefixOf (x:xs) (y:ys) = (x ==# y) && isPrefixOf xs ys
        {
            let ipf_id_chirho = self.resolve_or_fresh_id_chirho("isPrefixOf");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let scr1_chirho = self.fresh_binder_chirho("_p1", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_p2", list_a_chirho.clone());
            let xh_chirho = self.fresh_binder_chirho("xh", a_chirho.clone());
            let xt_chirho = self.fresh_binder_chirho("xt", list_a_chirho.clone());
            let yh_chirho = self.fresh_binder_chirho("yh", a_chirho.clone());
            let yt_chirho = self.fresh_binder_chirho("yt", list_a_chirho.clone());
            let eq_scr_chirho = self.fresh_binder_chirho("_eq", TyChirho::bool_chirho());

            // isPrefixOf xt yt
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(ipf_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xt_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(yt_chirho.id_chirho)),
            };

            // xh ==# yh
            let eq_check_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(xh_chirho.id_chirho),
                    CoreExprChirho::VarChirho(yh_chirho.id_chirho),
                ],
            };

            // case (xh ==# yh) of { True -> isPrefixOf xt yt; False -> False }
            let and_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_check_chirho),
                bind_chirho: eq_scr_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };

            // case ys of { [] -> False; (y:ys) -> ... }
            let case_ys_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![yh_chirho, yt_chirho],
                        rhs_chirho: and_check_chirho,
                    },
                ],
            };

            // case xs of { [] -> True; (x:xs) -> case ys ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr1_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![xh_chirho, xt_chirho],
                        rhs_chirho: case_ys_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: ipf_id_chirho,
                    name_chirho: "isPrefixOf".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // isSuffixOf :: [Int] -> [Int] -> Bool
        // isSuffixOf xs ys = isPrefixOf (reverse xs) (reverse ys)
        {
            let isf_id_chirho = self.resolve_or_fresh_id_chirho("isSuffixOf");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let ipf_id_chirho = self.resolve_or_fresh_id_chirho("isPrefixOf");
            let rev_id_chirho = self.resolve_or_fresh_id_chirho("reverse");

            let rev_xs_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(rev_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let rev_ys_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(rev_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(ipf_id_chirho)),
                    arg_chirho: Box::new(rev_xs_chirho),
                }),
                arg_chirho: Box::new(rev_ys_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: isf_id_chirho,
                    name_chirho: "isSuffixOf".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        TyChirho::fun_chirho(list_a_chirho.clone(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // unzip3 :: [(a,b,c)] -> ([a],[b],[c])
        // unzip3 [] = ([], [], [])
        // unzip3 ((a,b,c):rest) = let (as,bs,cs) = unzip3 rest in (a:as, b:bs, c:cs)
        {
            let uz3_id_chirho = self.resolve_or_fresh_id_chirho("unzip3");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_uz", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let tscr_chirho = self.fresh_binder_chirho("_th", a_chirho.clone());
            let ta_chirho = self.fresh_binder_chirho("ta", a_chirho.clone());
            let tb_chirho = self.fresh_binder_chirho("tb", b_chirho.clone());
            let tc_chirho = self.fresh_binder_chirho("tc", c_chirho.clone());
            let rscr_chirho = self.fresh_binder_chirho("_rr", a_chirho.clone());
            let ra_chirho = self.fresh_binder_chirho("ra", list_a_chirho.clone());
            let rb_chirho = self.fresh_binder_chirho("rb", list_b_chirho.clone());
            let rc_chirho = self.fresh_binder_chirho("rc", list_a_chirho.clone());

            // unzip3 t
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(uz3_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // case rec of { $tuple3 ra rb rc -> (ta:ra, tb:rb, tc:rc) }
            let result_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![
                    cons_chirho(
                        CoreExprChirho::VarChirho(ta_chirho.id_chirho),
                        CoreExprChirho::VarChirho(ra_chirho.id_chirho),
                    ),
                    cons_chirho(
                        CoreExprChirho::VarChirho(tb_chirho.id_chirho),
                        CoreExprChirho::VarChirho(rb_chirho.id_chirho),
                    ),
                    cons_chirho(
                        CoreExprChirho::VarChirho(tc_chirho.id_chirho),
                        CoreExprChirho::VarChirho(rc_chirho.id_chirho),
                    ),
                ],
            };

            let case_rec_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(rec_chirho),
                bind_chirho: rscr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple3".to_string()),
                    binders_chirho: vec![ra_chirho, rb_chirho, rc_chirho],
                    rhs_chirho: result_tuple_chirho,
                }],
            };

            // case h of { $tuple3 ta tb tc -> case rec ... }
            let case_head_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                bind_chirho: tscr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple3".to_string()),
                    binders_chirho: vec![ta_chirho, tb_chirho, tc_chirho],
                    rhs_chirho: case_rec_chirho,
                }],
            };

            let empty_result_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![nil_chirho(), nil_chirho(), nil_chirho()],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: empty_result_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: case_head_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: uz3_id_chirho,
                    name_chirho: "unzip3".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Prelude functions that use Ord/Eq: elem, notElem, minimum, maximum, sort.
    /// These are specialized to Int for now using primitive comparison ops.
    fn generate_ord_prelude_chirho(&mut self) {
        let int_chirho = TyChirho::ConChirho("Int".to_string());
        let list_int_chirho = TyChirho::ListChirho(Box::new(int_chirho.clone()));

        // ── elem :: Int -> [Int] -> Bool ──
        // elem e [] = False
        // elem e (x:xs) = case e ==# x of True -> True; False -> elem e xs
        {
            let elem_id_chirho = self.resolve_or_fresh_id_chirho("elem");
            let e_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$wild2", list_int_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(elem_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let eq_check_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(e_chirho.id_chirho),
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    ],
                }),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild2_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: eq_check_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: elem_id_chirho,
                    name_chirho: "elem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), list_int_chirho.clone()],
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── notElem :: Int -> [Int] -> Bool ──
        // notElem e xs = case elem e xs of True -> False; False -> True
        {
            let notelem_id_chirho = self.resolve_or_fresh_id_chirho("notElem");
            let elem_id_chirho = self.resolve_or_fresh_id_chirho("elem");
            let e_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());

            let elem_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(elem_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(elem_call_chirho),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: notelem_id_chirho,
                    name_chirho: "notElem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), list_int_chirho.clone()],
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── minimum :: [Int] -> Int ──
        // minimum xs = foldl min (head xs) (tail xs)
        {
            let minimum_id_chirho = self.resolve_or_fresh_id_chirho("minimum");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let min_id_chirho = self.resolve_or_fresh_id_chirho("min");
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());

            let head_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(head_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let tail_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            // foldl min (head xs) (tail xs)
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(min_id_chirho)),
                    }),
                    arg_chirho: Box::new(head_call_chirho),
                }),
                arg_chirho: Box::new(tail_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: minimum_id_chirho,
                    name_chirho: "minimum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── maximum :: [Int] -> Int ──
        // maximum xs = foldl max (head xs) (tail xs)
        {
            let maximum_id_chirho = self.resolve_or_fresh_id_chirho("maximum");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");
            let max_id_chirho = self.resolve_or_fresh_id_chirho("max");
            let head_id_chirho = self.resolve_or_fresh_id_chirho("head");
            let tail_id_chirho = self.resolve_or_fresh_id_chirho("tail");
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());

            let head_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(head_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let tail_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tail_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(max_id_chirho)),
                    }),
                    arg_chirho: Box::new(head_call_chirho),
                }),
                arg_chirho: Box::new(tail_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: maximum_id_chirho,
                    name_chirho: "maximum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── sort :: [Int] -> [Int] (insertion sort via <=# primop) ──
        // sort [] = []
        // sort (x:xs) = insert x (sort xs)
        // insert e [] = [e]
        // insert e (x:xs) = case e <=# x of True -> e:x:xs; False -> x : insert e xs
        {
            let insert_id_chirho = self.resolve_or_fresh_id_chirho("insert");
            let sort_id_chirho = self.resolve_or_fresh_id_chirho("sort");

            // ── insert ──
            let e_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_int_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", list_int_chirho.clone());
            let wild2_chirho = self.fresh_binder_chirho("$wild2", TyChirho::bool_chirho());

            let singleton_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(),
                        args_chirho: vec![],
                    },
                ],
            };
            // e : y : rest
            let e_cons_all_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(e_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(y_chirho.id_chirho),
                            CoreExprChirho::VarChirho(rest_chirho.id_chirho),
                        ],
                    },
                ],
            };
            // y : insert e rest
            let rec_insert_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(e_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
                    },
                ],
            };
            let le_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(e_chirho.id_chirho),
                        CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    ],
                }),
                bind_chirho: wild2_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: e_cons_all_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_insert_chirho,
                    },
                ],
            };
            let insert_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![y_chirho, rest_chirho],
                        rhs_chirho: le_case_chirho,
                    },
                ],
            };
            let insert_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: e_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho,
                    body_chirho: Box::new(insert_body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: insert_id_chirho,
                    name_chirho: "insert".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), list_int_chirho.clone()],
                        list_int_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: insert_rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // ── sort ──
            let xs_chirho = self.fresh_binder_chirho("xs", list_int_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_int_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", list_int_chirho.clone());

            let sort_rest_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(sort_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let insert_into_sorted_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(insert_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(sort_rest_chirho),
            };
            let sort_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: list_int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: insert_into_sorted_chirho,
                    },
                ],
            };
            let sort_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(sort_body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: sort_id_chirho,
                    name_chirho: "sort".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_int_chirho.clone(),
                        list_int_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: sort_rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // abs is already defined in generate_prelude_bindings_chirho

        // ── signum :: Int -> Int ──
        // signum n = case n ># 0 of True -> 1; False -> case n <# 0 of True -> -1; False -> 0
        {
            let signum_id_chirho = self.resolve_or_fresh_id_chirho("signum");
            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());
            let wild1_chirho = self.fresh_binder_chirho("$w1", TyChirho::bool_chirho());
            let wild2_chirho = self.fresh_binder_chirho("$w2", TyChirho::bool_chirho());

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild2_chirho,
                result_ty_chirho: int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(-1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: ">#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild1_chirho,
                result_ty_chirho: int_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: signum_id_chirho,
                    name_chirho: "signum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: rhs_chirho.clone(),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // Also generate $prim_Num_signum_Int for the instance dictionary
            let prim_signum_name_chirho = "$prim_Num_signum_Int";
            let prim_signum_id_chirho = self.resolve_or_fresh_id_chirho(prim_signum_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_signum_id_chirho,
                    name_chirho: prim_signum_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_chirho.clone(), int_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // even and odd are already defined in generate_prelude_bindings_chirho

        // ── replicate :: Int -> a -> [a] ──
        // replicate 0 _ = []
        // replicate n x = x : replicate (n-1) x
        {
            let replicate_id_chirho = self.resolve_or_fresh_id_chirho("replicate");
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9995));
            let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));

            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$wild", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(replicate_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "-#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(n_chirho.id_chirho),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<=#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(n_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: wild_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: replicate_id_chirho,
                    name_chirho: "replicate".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![int_chirho.clone(), a_chirho],
                        list_a_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate additional list Prelude functions: takeWhile, dropWhile, iterate, lookup, unzip, scanl.
    fn generate_extra_list_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };

        // ── takeWhile :: (a -> Bool) -> [a] -> [a] ──
        // takeWhile p [] = []; takeWhile p (x:xs) = if p x then x : takeWhile p xs else []
        {
            let tw_id_chirho = self.resolve_or_fresh_id_chirho("takeWhile");
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(tw_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let cons_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tw_id_chirho,
                    name_chirho: "takeWhile".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                            list_a_chirho.clone(),
                        ],
                        list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── dropWhile :: (a -> Bool) -> [a] -> [a] ──
        // dropWhile p [] = []; dropWhile p (x:xs) = if p x then dropWhile p xs else x:xs
        {
            let dw_id_chirho = self.resolve_or_fresh_id_chirho("dropWhile");
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(dw_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let keep_rest_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    CoreExprChirho::VarChirho(t_chirho.id_chirho),
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: keep_rest_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: dw_id_chirho,
                    name_chirho: "dropWhile".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                            list_a_chirho.clone(),
                        ],
                        list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── iterate :: (a -> a) -> a -> [a] ──
        // iterate f x = x : iterate f (f x)
        {
            let iter_id_chirho = self.resolve_or_fresh_id_chirho("iterate");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
            );
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let f_x_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(iter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(f_x_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            rec_call_chirho,
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: iter_id_chirho,
                    name_chirho: "iterate".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                            a_chirho.clone(),
                        ],
                        list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── lookup :: a -> [(a,b)] -> Maybe b ──  (uses ==# for Int keys)
        // lookup _ [] = Nothing; lookup k ((k',v):rest) = if k ==# k' then Just v else lookup k rest
        {
            let lookup_id_chirho = self.resolve_or_fresh_id_chirho("lookup");
            let pair_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let list_pair_chirho = TyChirho::ListChirho(Box::new(pair_ty_chirho.clone()));
            let maybe_b_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(b_chirho.clone()),
            );
            let k_chirho = self.fresh_binder_chirho("k", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_pair_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", pair_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_pair_chirho.clone());
            let kp_chirho = self.fresh_binder_chirho("k'", a_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", b_chirho.clone());
            let w1_chirho = self.fresh_binder_chirho("$w1", list_pair_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", pair_ty_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(lookup_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(k_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let eq_check_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(k_chirho.id_chirho),
                    CoreExprChirho::VarChirho(kp_chirho.id_chirho),
                ],
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_check_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: maybe_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Just".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(v_chirho.id_chirho)],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };
            // Destructure the pair
            let pair_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: maybe_b_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![kp_chirho, v_chirho],
                    rhs_chirho: cond_chirho,
                }],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w1_chirho,
                result_ty_chirho: maybe_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Nothing".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![pair_chirho, rest_chirho],
                        rhs_chirho: pair_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: lookup_id_chirho,
                    name_chirho: "lookup".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![a_chirho.clone(), list_pair_chirho],
                        maybe_b_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── unzip :: [(a,b)] -> ([a],[b]) ──
        // unzip [] = ([],[]); unzip ((a,b):rest) = let (as,bs) = unzip rest in (a:as, b:bs)
        {
            let unzip_id_chirho = self.resolve_or_fresh_id_chirho("unzip");
            let pair_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let list_pair_chirho = TyChirho::ListChirho(Box::new(pair_ty_chirho.clone()));
            let result_ty_chirho = TyChirho::ConChirho("(,)".to_string());

            let xs_chirho = self.fresh_binder_chirho("xs", list_pair_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", pair_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_pair_chirho.clone());
            let pa_chirho = self.fresh_binder_chirho("pa", a_chirho.clone());
            let pb_chirho = self.fresh_binder_chirho("pb", b_chirho.clone());
            let w1_chirho = self.fresh_binder_chirho("$w1", list_pair_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", pair_ty_chirho.clone());
            let res_chirho = self.fresh_binder_chirho("res", result_ty_chirho.clone());
            let as_chirho = self.fresh_binder_chirho("as_", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs_", list_b_chirho.clone());
            let w3_chirho = self.fresh_binder_chirho("$w3", result_ty_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(unzip_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            // let res = unzip rest in case res of (as_,bs_) -> (pa:as_, pb:bs_)
            let result_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(res_chirho.id_chirho)),
                bind_chirho: w3_chirho,
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![as_chirho.clone(), bs_chirho.clone()],
                    rhs_chirho: CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::ConAppChirho {
                                con_name_chirho: ":".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(pa_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(as_chirho.id_chirho),
                                ],
                            },
                            CoreExprChirho::ConAppChirho {
                                con_name_chirho: ":".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(pb_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(bs_chirho.id_chirho),
                                ],
                            },
                        ],
                    },
                }],
            };
            let let_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(res_chirho, rec_call_chirho)],
                body_chirho: Box::new(result_case_chirho),
            };
            // Destructure pair
            let pair_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: w2_chirho,
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![pa_chirho, pb_chirho],
                    rhs_chirho: let_body_chirho,
                }],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w1_chirho,
                result_ty_chirho: result_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![nil_chirho(), nil_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![pair_chirho, rest_chirho],
                        rhs_chirho: pair_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: unzip_id_chirho,
                    name_chirho: "unzip".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_pair_chirho, result_ty_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── scanl :: (b -> a -> b) -> b -> [a] -> [b] ──
        // scanl f z [] = [z]; scanl f z (x:xs) = z : scanl f (f z x) xs
        {
            let scanl_id_chirho = self.resolve_or_fresh_id_chirho("scanl");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_n_chirho(vec![b_chirho.clone(), a_chirho.clone()], b_chirho.clone()),
            );
            let z_chirho = self.fresh_binder_chirho("z", b_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());

            let f_z_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(scanl_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(f_z_h_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: list_b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: ":".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                nil_chirho(),
                            ],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: ":".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                rec_call_chirho,
                            ],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: scanl_id_chirho,
                    name_chirho: "scanl".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_n_chirho(
                                vec![b_chirho.clone(), a_chirho.clone()],
                                b_chirho.clone(),
                            ),
                            b_chirho.clone(),
                            list_a_chirho.clone(),
                        ],
                        list_b_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── span :: (a -> Bool) -> [a] -> ([a],[a]) ──
        // span p [] = ([],[]); span p (x:xs) = if p x then let (ys,zs) = span p xs in (x:ys,zs) else ([],x:xs)
        {
            let span_id_chirho = self.resolve_or_fresh_id_chirho("span");
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());
            let res_chirho = self.fresh_binder_chirho("res", tuple_ty_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", tuple_ty_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            // let res = span p t in case res of (ys,zs) -> (h:ys, zs)
            let true_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(res_chirho.clone(), rec_call_chirho)],
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(res_chirho.id_chirho)),
                    bind_chirho: w2_chirho,
                    result_ty_chirho: tuple_ty_chirho.clone(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                        binders_chirho: vec![ys_chirho.clone(), zs_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::ConAppChirho {
                                    con_name_chirho: ":".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(h_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                                    ],
                                },
                                CoreExprChirho::VarChirho(zs_chirho.id_chirho),
                            ],
                        },
                    }],
                }),
            };
            // False branch: ([], x:xs)
            let false_body_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    nil_chirho(),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            CoreExprChirho::VarChirho(t_chirho.id_chirho),
                        ],
                    },
                ],
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: true_body_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: false_body_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![nil_chirho(), nil_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cond_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: span_id_chirho,
                    name_chirho: "span".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                            list_a_chirho.clone(),
                        ],
                        tuple_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // break = span . not
            // break p = span (not . p)
            let break_id_chirho = self.resolve_or_fresh_id_chirho("break");
            let bp_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let bxs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let bx_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            // \x -> not (p x) — uses case to negate
            let bwb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());
            let not_p_x_chirho = CoreExprChirho::LamChirho {
                binder_chirho: bx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(bp_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(bx_chirho.id_chirho)),
                    }),
                    bind_chirho: bwb_chirho,
                    result_ty_chirho: TyChirho::bool_chirho(),
                    alts_chirho: vec![
                        CoreAltChirho {
                            con_chirho: AltConChirho::DataConChirho("True".to_string()),
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::ConAppChirho {
                                con_name_chirho: "False".to_string(),
                                args_chirho: vec![],
                            },
                        },
                        CoreAltChirho {
                            con_chirho: AltConChirho::DefaultChirho,
                            binders_chirho: vec![],
                            rhs_chirho: CoreExprChirho::ConAppChirho {
                                con_name_chirho: "True".to_string(),
                                args_chirho: vec![],
                            },
                        },
                    ],
                }),
            };
            let break_body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(not_p_x_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(bxs_chirho.id_chirho)),
            };
            let break_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: bp_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: bxs_chirho,
                    body_chirho: Box::new(break_body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: break_id_chirho,
                    name_chirho: "break".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                            list_a_chirho.clone(),
                        ],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: break_rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── partition :: (a -> Bool) -> [a] -> ([a],[a]) ──
        // partition p [] = ([],[]); partition p (x:xs) = let (yes,no) = partition p xs
        //   in if p x then (x:yes, no) else (yes, x:no)
        {
            let part_id_chirho = self.resolve_or_fresh_id_chirho("partition");
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let p_chirho = self.fresh_binder_chirho(
                "p",
                TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", list_a_chirho.clone());
            let wb_chirho = self.fresh_binder_chirho("$wb", TyChirho::bool_chirho());
            let res_chirho = self.fresh_binder_chirho("res", tuple_ty_chirho.clone());
            let yes_chirho = self.fresh_binder_chirho("yes", list_a_chirho.clone());
            let no_chirho = self.fresh_binder_chirho("no", list_a_chirho.clone());
            let w2_chirho = self.fresh_binder_chirho("$w2", tuple_ty_chirho.clone());

            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(part_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let p_h_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            // true: (h:yes, no)
            let true_rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            CoreExprChirho::VarChirho(yes_chirho.id_chirho),
                        ],
                    },
                    CoreExprChirho::VarChirho(no_chirho.id_chirho),
                ],
            };
            // false: (yes, h:no)
            let false_rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(yes_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: ":".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(h_chirho.id_chirho),
                            CoreExprChirho::VarChirho(no_chirho.id_chirho),
                        ],
                    },
                ],
            };
            let cond_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_h_chirho),
                bind_chirho: wb_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: true_rhs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: false_rhs_chirho,
                    },
                ],
            };
            let cons_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(res_chirho.clone(), rec_call_chirho)],
                body_chirho: Box::new(CoreExprChirho::CaseChirho {
                    scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(res_chirho.id_chirho)),
                    bind_chirho: w2_chirho,
                    result_ty_chirho: tuple_ty_chirho.clone(),
                    alts_chirho: vec![CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                        binders_chirho: vec![yes_chirho, no_chirho],
                        rhs_chirho: cond_chirho,
                    }],
                }),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: w_chirho,
                result_ty_chirho: tuple_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![nil_chirho(), nil_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_body_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: part_id_chirho,
                    name_chirho: "partition".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(a_chirho.clone(), TyChirho::bool_chirho()),
                            list_a_chirho.clone(),
                        ],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate `$prim_Enum_*` and `$prim_Bounded_*` bindings for instance dictionaries.
    fn generate_enum_bounded_prelude_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();

        // ── Enum Int: toEnum = identity, fromEnum = identity ──
        {
            let prim_name_chirho = "$prim_Enum_toEnum_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
        {
            let prim_name_chirho = "$prim_Enum_fromEnum_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Enum Int: succ = (+1), pred = (-1) ──
        {
            let prim_name_chirho = "$prim_Enum_succ_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "+#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
        {
            let prim_name_chirho = "$prim_Enum_pred_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "-#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    ],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Enum Char: toEnum = chr-like, fromEnum = ord-like ──
        // For Char, toEnum and fromEnum are identity on the underlying Int representation
        {
            let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
            let prim_name_chirho = "$prim_Enum_toEnum_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), char_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            let prim_name_chirho = "$prim_Enum_fromEnum_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "ord#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho, int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Enum Bool: toEnum 0 = False, toEnum _ = True; fromEnum False = 0, fromEnum True = 1 ──
        {
            let bool_ty_chirho = TyChirho::bool_chirho();

            // toEnum for Bool: \n -> case n ==# 0 of { True -> False; _ -> True }
            let prim_name_chirho = "$prim_Enum_toEnum_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let eq_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "==#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                ],
            };
            let wild_chirho = self.fresh_binder_chirho("$w", bool_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(eq_zero_chirho),
                bind_chirho: wild_chirho,
                result_ty_chirho: bool_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "False".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "True".to_string(),
                            args_chirho: vec![],
                        },
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), bool_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // fromEnum for Bool: \b -> case b of { False -> 0; True -> 1 }
            let prim_name_chirho = "$prim_Enum_fromEnum_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let b_chirho = self.fresh_binder_chirho("b", bool_ty_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("$w", bool_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: b_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(bool_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Bounded Int: minBound = MIN, maxBound = MAX ──
        {
            let prim_name_chirho = "$prim_Bounded_minBound_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: int_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MIN)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
        {
            let prim_name_chirho = "$prim_Bounded_maxBound_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: int_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MAX)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Bounded Char ──
        {
            let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
            let prim_name_chirho = "$prim_Bounded_minBound_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: char_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
            let prim_name_chirho = "$prim_Bounded_maxBound_Char";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: char_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0x10FFFF)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Bounded Bool ──
        {
            let bool_ty_chirho = TyChirho::bool_chirho();
            let prim_name_chirho = "$prim_Bounded_minBound_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: bool_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::ConAppChirho {
                    con_name_chirho: "False".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
            let prim_name_chirho = "$prim_Bounded_maxBound_Bool";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: bool_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::ConAppChirho {
                    con_name_chirho: "True".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Also generate prelude-level toEnum/fromEnum/succ/pred bindings
        // so they can be used as regular functions
        {
            // toEnum :: Int -> Int (defaulting to Int-specialized)
            let to_enum_id_chirho = self.resolve_or_fresh_id_chirho("toEnum");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: to_enum_id_chirho,
                    name_chirho: "toEnum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // fromEnum :: Int -> Int (defaulting to Int-specialized)
            let from_enum_id_chirho = self.resolve_or_fresh_id_chirho("fromEnum");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: from_enum_id_chirho,
                    name_chirho: "fromEnum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // succ :: Int -> Int = \x -> x +# 1
            let succ_id_chirho = self.resolve_or_fresh_id_chirho("succ");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: succ_id_chirho,
                    name_chirho: "succ".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // pred :: Int -> Int = \x -> x -# 1
            let pred_id_chirho = self.resolve_or_fresh_id_chirho("pred");
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: pred_id_chirho,
                    name_chirho: "pred".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "-#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                        ],
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // minBound :: Int (prelude-level, defaulting to Int)
            let min_bound_id_chirho = self.resolve_or_fresh_id_chirho("minBound");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: min_bound_id_chirho,
                    name_chirho: "minBound".to_string(),
                    ty_chirho: int_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MIN)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // maxBound :: Int (prelude-level, defaulting to Int)
            let max_bound_id_chirho = self.resolve_or_fresh_id_chirho("maxBound");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: max_bound_id_chirho,
                    name_chirho: "maxBound".to_string(),
                    ty_chirho: int_ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(i64::MAX)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate `$prim_Integral_*` bindings for Integral Int instance dictionary.
    fn generate_integral_prelude_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();
        let int2_int_chirho = TyChirho::fun_n_chirho(
            [int_ty_chirho.clone(), int_ty_chirho.clone()],
            int_ty_chirho.clone(),
        );

        // $prim_Integral_div_Int = \a b -> div# a b
        for (method_chirho, primop_chirho) in [
            ("div", "div#"),
            ("mod", "mod#"),
            ("quot", "quot#"),
            ("rem", "rem#"),
        ] {
            let prim_name_chirho = format!("$prim_Integral_{}_Int", method_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_chirho.to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho,
                    ty_chirho: int2_int_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Integral_toInteger_Int = identity
        {
            let prim_name_chirho = "$prim_Integral_toInteger_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Integral_quotRem_Int = \a b -> (quot# a b, rem# a b)
        {
            let prim_name_chirho = "$prim_Integral_quotRem_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "quot#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "rem#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        tuple_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Integral_divMod_Int = \a b -> (div# a b, mod# a b)
        {
            let prim_name_chirho = "$prim_Integral_divMod_Int";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "div#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "mod#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                ],
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level quot and rem bindings
        {
            let quot_id_chirho = self.resolve_or_fresh_id_chirho("quot");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: quot_id_chirho,
                    name_chirho: "quot".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        int_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "quot#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                CoreExprChirho::VarChirho(b_chirho.id_chirho),
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            let rem_id_chirho = self.resolve_or_fresh_id_chirho("rem");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: rem_id_chirho,
                    name_chirho: "rem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        int_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "rem#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                CoreExprChirho::VarChirho(b_chirho.id_chirho),
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level divMod and quotRem: tuple wrappers over the same
        // primops as their scalar counterparts, so they inherit the correct
        // flooring div#/mod# and truncating quot#/rem# semantics everywhere.
        {
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());

            let divmod_id_chirho = self.resolve_or_fresh_id_chirho("divMod");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: divmod_id_chirho,
                    name_chirho: "divMod".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        tuple_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::PrimOpChirho {
                                    name_chirho: "div#".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                    ],
                                },
                                CoreExprChirho::PrimOpChirho {
                                    name_chirho: "mod#".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                    ],
                                },
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            let quotrem_id_chirho = self.resolve_or_fresh_id_chirho("quotRem");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: quotrem_id_chirho,
                    name_chirho: "quotRem".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), int_ty_chirho.clone()],
                        tuple_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::PrimOpChirho {
                                    name_chirho: "quot#".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                    ],
                                },
                                CoreExprChirho::PrimOpChirho {
                                    name_chirho: "rem#".to_string(),
                                    args_chirho: vec![
                                        CoreExprChirho::VarChirho(a_chirho.id_chirho),
                                        CoreExprChirho::VarChirho(b_chirho.id_chirho),
                                    ],
                                },
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level splitAt n xs = (take n xs, drop n xs), reusing the
        // existing take/drop bindings so behaviour stays consistent.
        {
            let list_ty_chirho = TyChirho::string_chirho(); // placeholder list type (see take/drop)
            let tuple_ty_chirho = TyChirho::ConChirho("(,)".to_string());
            let take_id_chirho = self.resolve_or_fresh_id_chirho("take");
            let drop_id_chirho = self.resolve_or_fresh_id_chirho("drop");
            let splitat_id_chirho = self.resolve_or_fresh_id_chirho("splitAt");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: splitat_id_chirho,
                    name_chirho: "splitAt".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        [int_ty_chirho.clone(), list_ty_chirho.clone()],
                        tuple_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: n_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: xs_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                            con_name_chirho: "$tuple2".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            take_id_chirho,
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            n_chirho.id_chirho,
                                        )),
                                    }),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                        xs_chirho.id_chirho,
                                    )),
                                },
                                CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            drop_id_chirho,
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            n_chirho.id_chirho,
                                        )),
                                    }),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                        xs_chirho.id_chirho,
                                    )),
                                },
                            ],
                        }),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level gcd binding.
        // gcd a b = go (abs a) (abs b), inlined to avoid depending on local lets.
        {
            let gcd_id_chirho = self.resolve_or_fresh_id_chirho("gcd");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let var_chirho =
                |binder_chirho: &BinderChirho| CoreExprChirho::VarChirho(binder_chirho.id_chirho);
            let gcd_call_chirho =
                |left_chirho: CoreExprChirho, right_chirho: CoreExprChirho| -> CoreExprChirho {
                    CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(gcd_id_chirho)),
                            arg_chirho: Box::new(left_chirho),
                        }),
                        arg_chirho: Box::new(right_chirho),
                    }
                };
            let negate_chirho = |expr_chirho: CoreExprChirho| CoreExprChirho::PrimOpChirho {
                name_chirho: "negate#".to_string(),
                args_chirho: vec![expr_chirho],
            };

            let b_zero_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        var_chirho(&b_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: self.fresh_binder_chirho("$w", TyChirho::bool_chirho()),
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: var_chirho(&a_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: gcd_call_chirho(
                            var_chirho(&b_chirho),
                            CoreExprChirho::PrimOpChirho {
                                name_chirho: "rem#".to_string(),
                                args_chirho: vec![var_chirho(&a_chirho), var_chirho(&b_chirho)],
                            },
                        ),
                    },
                ],
            };

            let b_negative_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        var_chirho(&b_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: self.fresh_binder_chirho("$w", TyChirho::bool_chirho()),
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: gcd_call_chirho(
                            var_chirho(&a_chirho),
                            negate_chirho(var_chirho(&b_chirho)),
                        ),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: b_zero_case_chirho,
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        var_chirho(&a_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: self.fresh_binder_chirho("$w", TyChirho::bool_chirho()),
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: gcd_call_chirho(
                            negate_chirho(var_chirho(&a_chirho)),
                            var_chirho(&b_chirho),
                        ),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: b_negative_case_chirho,
                    },
                ],
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: gcd_id_chirho,
                    name_chirho: "gcd".to_string(),
                    ty_chirho: int2_int_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                },
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level lcm binding.
        // lcm a b = if a == 0 || b == 0 then 0 else abs ((a `quot` gcd a b) * b)
        {
            let lcm_id_chirho = self.resolve_or_fresh_id_chirho("lcm");
            let gcd_id_chirho = self.resolve_or_fresh_id_chirho("gcd");
            let a_chirho = self.fresh_binder_chirho("a", int_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", int_ty_chirho.clone());
            let var_chirho =
                |binder_chirho: &BinderChirho| CoreExprChirho::VarChirho(binder_chirho.id_chirho);
            let gcd_call_chirho = || CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(gcd_id_chirho)),
                    arg_chirho: Box::new(var_chirho(&a_chirho)),
                }),
                arg_chirho: Box::new(var_chirho(&b_chirho)),
            };
            let product_chirho = || CoreExprChirho::PrimOpChirho {
                name_chirho: "*#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "quot#".to_string(),
                        args_chirho: vec![var_chirho(&a_chirho), gcd_call_chirho()],
                    },
                    var_chirho(&b_chirho),
                ],
            };
            let abs_product_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "<#".to_string(),
                    args_chirho: vec![
                        product_chirho(),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: self.fresh_binder_chirho("$w", TyChirho::bool_chirho()),
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "negate#".to_string(),
                            args_chirho: vec![product_chirho()],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: product_chirho(),
                    },
                ],
            };
            let b_zero_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        var_chirho(&b_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: self.fresh_binder_chirho("$w", TyChirho::bool_chirho()),
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: abs_product_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        var_chirho(&a_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    ],
                }),
                bind_chirho: self.fresh_binder_chirho("$w", TyChirho::bool_chirho()),
                result_ty_chirho: int_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: b_zero_case_chirho,
                    },
                ],
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: lcm_id_chirho,
                    name_chirho: "lcm".to_string(),
                    ty_chirho: int2_int_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Data.Char prelude bindings: chr, ord, isDigit, isAlpha, etc.
    fn generate_char_prelude_chirho(&mut self) {
        let int_ty_chirho = TyChirho::int_chirho();
        let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
        let bool_ty_chirho = TyChirho::bool_chirho();

        // Unary Char->Bool functions
        for (name_chirho, primop_chirho) in [
            ("isDigit", "isDigit#"),
            ("isAlpha", "isAlpha#"),
            ("isAlphaNum", "isAlphaNum#"),
            ("isUpper", "isUpper#"),
            ("isLower", "isLower#"),
            ("isSpace", "isSpace#"),
        ] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho.clone(), bool_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Unary Char->Char functions
        for (name_chirho, primop_chirho) in [("toLower", "toLower#"), ("toUpper", "toUpper#")] {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho.clone(), char_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // chr :: Int -> Char
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("chr");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "chr#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "chr".to_string(),
                    ty_chirho: TyChirho::fun_chirho(int_ty_chirho.clone(), char_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ord :: Char -> Int
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("ord");
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "ord#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "ord".to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // digitToInt :: Char -> Int
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("digitToInt");
            let c_chirho = self.fresh_binder_chirho("c", char_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "digitToInt#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "digitToInt".to_string(),
                    ty_chirho: TyChirho::fun_chirho(char_ty_chirho, int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // intToDigit :: Int -> Char
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("intToDigit");
            let n_chirho = self.fresh_binder_chirho("n", int_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "intToDigit#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "intToDigit".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        int_ty_chirho,
                        TyChirho::ConChirho("Char".to_string()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    fn generate_floating_prelude_chirho(&mut self) {
        let double_ty_chirho = TyChirho::double_chirho();
        let d2d_chirho = TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone());

        // Unary floating functions: $prim_Floating_<method>_Double = \x -> <method># x
        // Plus prelude-level bindings: sin = $prim_Floating_sin_Double, etc.
        for (method_chirho, primop_chirho) in [
            ("sin", "sin#"),
            ("cos", "cos#"),
            ("tan", "tan#"),
            ("asin", "asin#"),
            ("acos", "acos#"),
            ("atan", "atan#"),
            ("exp", "exp#"),
            ("log", "log#"),
            ("sqrt", "sqrt#"),
        ] {
            let prim_name_chirho = format!("$prim_Floating_{}_Double", method_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: primop_chirho.to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.clone(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // Prelude-level alias: sin = $prim_Floating_sin_Double
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(method_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: method_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(prim_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // pi :: Double (constant via pi# primop, wrapped as a thunk)
        {
            let prim_name_chirho = "$prim_Floating_pi_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            // pi# is dispatched as a unary primop that ignores its argument
            let dummy_chirho = self.fresh_binder_chirho("u", TyChirho::ConChirho("()".to_string()));
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: dummy_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "pi#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(dummy_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ConChirho("()".to_string()),
                        double_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // NOTE: We intentionally do NOT create a top-level "pi" alias
            // because user code may define local "pi" bindings (e.g. `where pi = 3`).
            // The $prim_Floating_pi_Double binding is available for typeclass dispatch.
        }

        // ── Num Double: abs and signum ──
        // $prim_Num_abs_Double = \n -> if n <. 0.0 then negateFloat# n else n
        {
            let prim_name_chirho = "$prim_Num_abs_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());
            let lt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<.#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                ],
            };
            let negated_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "negateFloat#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
            };
            let case_wild_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(lt_zero_chirho),
                bind_chirho: case_wild_chirho,
                result_ty_chirho: double_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: negated_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: n_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Num_signum_Double = \n -> if n < 0.0 then -1.0 elif n > 0.0 then 1.0 else 0.0
        {
            let prim_name_chirho = "$prim_Num_signum_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());

            let lt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "<.#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                ],
            };
            let gt_zero_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: ">.#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(n_chirho.id_chirho),
                    CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                ],
            };

            let w1_chirho = self.fresh_binder_chirho("$w1", TyChirho::bool_chirho());
            let w2_chirho = self.fresh_binder_chirho("$w2", TyChirho::bool_chirho());

            // Inner case: if n > 0.0 then 1.0 else 0.0
            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(gt_zero_chirho),
                bind_chirho: w2_chirho,
                result_ty_chirho: double_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(1.0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(0.0)),
                    },
                ],
            };

            // Outer case: if n < 0.0 then -1.0 else (inner)
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(lt_zero_chirho),
                bind_chirho: w1_chirho,
                result_ty_chirho: double_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::FloatChirho(-1.0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: inner_case_chirho,
                    },
                ],
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: n_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Num_fromInteger_Double = \n -> fromIntegral# n
        {
            let prim_name_chirho = "$prim_Num_fromInteger_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "fromIntegral#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        double_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Num Double: binary arithmetic ops ──
        // $prim_Num_+_Double = \a b -> +.# a b
        // $prim_Num_-_Double = \a b -> -.# a b
        // $prim_Num_*_Double = \a b -> *.# a b
        for (method_chirho, primop_chirho) in [("+", "+.#"), ("-", "-.#"), ("*", "*.#")] {
            let prim_name_chirho = format!("$prim_Num_{}_Double", method_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", double_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: primop_chirho.to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.clone(),
                    ty_chirho: TyChirho::fun_chirho(
                        double_ty_chirho.clone(),
                        TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Num_negate_Double = \n -> negateFloat# n
        {
            let prim_name_chirho = "$prim_Num_negate_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "negateFloat#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        double_ty_chirho.clone(),
                        double_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Fractional Double: /, recip, fromRational ──
        // $prim_Fractional_/_Double = \a b -> /.# a b
        {
            let prim_name_chirho = "$prim_Fractional_/_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", double_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "/.#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        double_ty_chirho.clone(),
                        TyChirho::fun_chirho(double_ty_chirho.clone(), double_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Fractional_recip_Double = \n -> recip# n
        {
            let prim_name_chirho = "$prim_Fractional_recip_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", double_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "recip#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: d2d_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Fractional_fromRational_Double = \n -> fromIntegral# n
        // (Simplified: treats Rational as Int for now, converting to Double)
        {
            let prim_name_chirho = "$prim_Fractional_fromRational_Double";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let n_chirho = self.fresh_binder_chirho("n", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: n_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "fromIntegral#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(n_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        double_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Floating Double prim bindings for instance dictionary ──
        // $prim_Floating_<method>_Double already generated above for sin/cos/tan/etc.
        // But the dict generation also needs them registered by this naming pattern.
        // The loop above already generates $prim_Floating_sin_Double etc.
        // We just need $prim_Floating_pi_Double which is already generated above.
    }

    fn generate_functor_monad_prelude_chirho(&mut self) {
        let any_ty_chirho = TyChirho::VarChirho(TyVarChirho(9999));

        // ── instance Functor Maybe ──
        // $prim_Functor_fmap_Maybe = \f -> \mx -> case mx of
        //     Nothing -> Nothing
        //     Just x  -> Just (f x)
        {
            let prim_name_chirho = "$prim_Functor_fmap_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: mx_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho.clone(),
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![x_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Just".to_string(),
                                    args_chirho: vec![CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            f_chirho.id_chirho,
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            x_chirho.id_chirho,
                                        )),
                                    }],
                                },
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // Prelude-level: fmap = $prim_Functor_fmap_Maybe (default to Maybe)
            let fmap_id_chirho = self.resolve_or_fresh_id_chirho("fmap");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fmap_id_chirho,
                    name_chirho: "fmap".to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(prim_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Functor [] ──
        // $prim_Functor_fmap_[] = map (reuse existing map binding)
        {
            let prim_name_chirho = "$prim_Functor_fmap_[]";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::VarChirho(map_id_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Functor Either (Left passthrough, Right maps) ──
        // $prim_Functor_fmap_Either = \f -> \ex -> case ex of
        //     Left a  -> Left a
        //     Right b -> Right (f b)
        {
            let prim_name_chirho = "$prim_Functor_fmap_Either";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let ex_chirho = self.fresh_binder_chirho("ex", any_ty_chirho.clone());
            let a_chirho = self.fresh_binder_chirho("a", any_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ex_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ex_chirho.id_chirho)),
                        bind_chirho: scrut_chirho.clone(),
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                                binders_chirho: vec![a_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Left".to_string(),
                                    args_chirho: vec![CoreExprChirho::VarChirho(
                                        a_chirho.id_chirho,
                                    )],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                                binders_chirho: vec![b_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Right".to_string(),
                                    args_chirho: vec![CoreExprChirho::AppChirho {
                                        fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                            f_chirho.id_chirho,
                                        )),
                                        arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                            b_chirho.id_chirho,
                                        )),
                                    }],
                                },
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Applicative Either ──
        // $prim_Applicative_pure_Either = \x -> Right x
        {
            let prim_name_chirho = "$prim_Applicative_pure_Either";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "Right".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Applicative_<*>_Either = \mf mx -> case mf of
        //     Left e  -> Left e
        //     Right f -> case mx of
        //         Left e  -> Left e
        //         Right x -> Right (f x)
        {
            let prim_name_chirho = "$prim_Applicative_<*>_Either";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mf_chirho = self.fresh_binder_chirho("mf", any_ty_chirho.clone());
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let outer_e_chirho = self.fresh_binder_chirho("e", any_ty_chirho.clone());
            let inner_e_chirho = self.fresh_binder_chirho("e", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut1_chirho = self.fresh_binder_chirho("_s1", any_ty_chirho.clone());
            let scrut2_chirho = self.fresh_binder_chirho("_s2", any_ty_chirho.clone());

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                bind_chirho: scrut2_chirho,
                result_ty_chirho: any_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                        binders_chirho: vec![inner_e_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Left".to_string(),
                            args_chirho: vec![CoreExprChirho::VarChirho(inner_e_chirho.id_chirho)],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Right".to_string(),
                            args_chirho: vec![CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                            }],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mf_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: mx_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mf_chirho.id_chirho)),
                        bind_chirho: scrut1_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                                binders_chirho: vec![outer_e_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Left".to_string(),
                                    args_chirho: vec![CoreExprChirho::VarChirho(
                                        outer_e_chirho.id_chirho,
                                    )],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                                binders_chirho: vec![f_chirho.clone()],
                                rhs_chirho: inner_case_chirho,
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Monad Either ──
        // $prim_Monad_>>=_Either = \mx f -> case mx of
        //     Left e  -> Left e
        //     Right x -> f x
        {
            let prim_name_chirho = "$prim_Monad_>>=_Either";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                                binders_chirho: vec![e_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Left".to_string(),
                                    args_chirho: vec![CoreExprChirho::VarChirho(
                                        e_chirho.id_chirho,
                                    )],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                                binders_chirho: vec![x_chirho.clone()],
                                rhs_chirho: CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                        f_chirho.id_chirho,
                                    )),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                        x_chirho.id_chirho,
                                    )),
                                },
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Monad_>>_Either = \mx my -> case mx of
        //     Left e  -> Left e
        //     Right _ -> my
        {
            let prim_name_chirho = "$prim_Monad_>>_Either";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let my_chirho = self.fresh_binder_chirho("my", any_ty_chirho.clone());
            let e_chirho = self.fresh_binder_chirho("e", any_ty_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("_w", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: my_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Left".to_string()),
                                binders_chirho: vec![e_chirho.clone()],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Left".to_string(),
                                    args_chirho: vec![CoreExprChirho::VarChirho(
                                        e_chirho.id_chirho,
                                    )],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Right".to_string()),
                                binders_chirho: vec![w_chirho.clone()],
                                rhs_chirho: CoreExprChirho::VarChirho(my_chirho.id_chirho),
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Functor/Applicative/Monad Identity ──
        // The imported Identity constructor is erased by the driver's imported
        // newtype-constructor seed, and runIdentity is body-backed above.
        {
            let prim_name_chirho = "$prim_Functor_fmap_Identity";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let prim_name_chirho = "$prim_Applicative_pure_Identity";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let prim_name_chirho = "$prim_Applicative_<*>_Identity";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let prim_name_chirho = "$prim_Monad_>>=_Identity";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let prim_name_chirho = "$prim_Monad_>>_Identity";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Applicative [] ──
        // $prim_Applicative_pure_[] = \x -> [x]
        {
            let prim_name_chirho = "$prim_Applicative_pure_[]";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: ":".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    ],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Applicative_<*>_[] = \fs xs -> concatMap (\f -> map f xs) fs
        {
            let prim_name_chirho = "$prim_Applicative_<*>_[]";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let concat_map_id_chirho = self.resolve_or_fresh_id_chirho("concatMap");
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            let fs_chirho = self.fresh_binder_chirho("fs", any_ty_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let map_f_xs_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(map_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: fs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(concat_map_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::LamChirho {
                                binder_chirho: f_chirho.clone(),
                                body_chirho: Box::new(map_f_xs_chirho),
                            }),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(fs_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Monad [] ──
        // $prim_Monad_>>=_[] = \xs f -> concatMap f xs
        {
            let prim_name_chirho = "$prim_Monad_>>=_[]";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let concat_map_id_chirho = self.resolve_or_fresh_id_chirho("concatMap");
            let xs_chirho = self.fresh_binder_chirho("xs", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(concat_map_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Monad_>>_[] = \xs ys -> concatMap (\_ -> ys) xs
        {
            let prim_name_chirho = "$prim_Monad_>>_[]";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let concat_map_id_chirho = self.resolve_or_fresh_id_chirho("concatMap");
            let xs_chirho = self.fresh_binder_chirho("xs", any_ty_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", any_ty_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("_w", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: ys_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(concat_map_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::LamChirho {
                                binder_chirho: w_chirho,
                                body_chirho: Box::new(CoreExprChirho::VarChirho(
                                    ys_chirho.id_chirho,
                                )),
                            }),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Applicative Maybe ──
        // $prim_Applicative_pure_Maybe = \x -> Just x
        {
            let prim_name_chirho = "$prim_Applicative_pure_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "Just".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Applicative_<*>_Maybe = \mf mx -> case mf of
        //     Nothing -> Nothing
        //     Just f  -> case mx of
        //         Nothing -> Nothing
        //         Just x  -> Just (f x)
        {
            let prim_name_chirho = "$prim_Applicative_<*>_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mf_chirho = self.fresh_binder_chirho("mf", any_ty_chirho.clone());
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut1_chirho = self.fresh_binder_chirho("_s1", any_ty_chirho.clone());
            let scrut2_chirho = self.fresh_binder_chirho("_s2", any_ty_chirho.clone());

            let inner_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                bind_chirho: scrut2_chirho,
                result_ty_chirho: any_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Nothing".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "Just".to_string(),
                            args_chirho: vec![CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                            }],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mf_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: mx_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mf_chirho.id_chirho)),
                        bind_chirho: scrut1_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![f_chirho.clone()],
                                rhs_chirho: inner_case_chirho,
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── instance Monad Maybe ──
        // $prim_Monad_>>=_Maybe = \mx f -> case mx of
        //     Nothing -> Nothing
        //     Just x  -> f x
        {
            let prim_name_chirho = "$prim_Monad_>>=_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let f_chirho = self.fresh_binder_chirho("f", any_ty_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![x_chirho.clone()],
                                rhs_chirho: CoreExprChirho::AppChirho {
                                    fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                        f_chirho.id_chirho,
                                    )),
                                    arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                        x_chirho.id_chirho,
                                    )),
                                },
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // $prim_Monad_>>_Maybe = \mx my -> case mx of
        //     Nothing -> Nothing
        //     Just _  -> my
        {
            let prim_name_chirho = "$prim_Monad_>>_Maybe";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let mx_chirho = self.fresh_binder_chirho("mx", any_ty_chirho.clone());
            let my_chirho = self.fresh_binder_chirho("my", any_ty_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("_w", any_ty_chirho.clone());
            let scrut_chirho = self.fresh_binder_chirho("_s", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: mx_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: my_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::CaseChirho {
                        scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(mx_chirho.id_chirho)),
                        bind_chirho: scrut_chirho,
                        result_ty_chirho: any_ty_chirho.clone(),
                        alts_chirho: vec![
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                                binders_chirho: vec![],
                                rhs_chirho: CoreExprChirho::ConAppChirho {
                                    con_name_chirho: "Nothing".to_string(),
                                    args_chirho: vec![],
                                },
                            },
                            CoreAltChirho {
                                con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                                binders_chirho: vec![w_chirho.clone()],
                                rhs_chirho: CoreExprChirho::VarChirho(my_chirho.id_chirho),
                            },
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: any_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate IO control flow Prelude functions:
    /// when, unless, mapM_, forM_, void, sequence_, guard
    fn generate_io_control_prelude_chirho(&mut self) {
        let unit_ty_chirho = TyChirho::unit_chirho();
        let bool_ty_chirho = TyChirho::bool_chirho();
        let any_ty_chirho = TyChirho::VarChirho(TyVarChirho(9999));
        let io_unit_chirho = unit_ty_chirho.clone(); // simplified IO model

        // when :: Bool -> IO () -> IO ()
        // when True  action = action
        // when False _      = return ()
        {
            let when_id_chirho = self.resolve_or_fresh_id_chirho("when");
            let cond_chirho = self.fresh_binder_chirho("cond", bool_ty_chirho.clone());
            let action_chirho = self.fresh_binder_chirho("action", io_unit_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(cond_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_w", bool_ty_chirho.clone()),
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(action_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                                0,
                            ))],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cond_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: action_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: when_id_chirho,
                    name_chirho: "when".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        bool_ty_chirho.clone(),
                        TyChirho::fun_chirho(io_unit_chirho.clone(), io_unit_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // unless :: Bool -> IO () -> IO ()
        // unless True  _      = return ()
        // unless False action = action
        {
            let unless_id_chirho = self.resolve_or_fresh_id_chirho("unless");
            let cond_chirho = self.fresh_binder_chirho("cond", bool_ty_chirho.clone());
            let action_chirho = self.fresh_binder_chirho("action", io_unit_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(cond_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_u", bool_ty_chirho.clone()),
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                                0,
                            ))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(action_chirho.id_chirho),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cond_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: action_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: unless_id_chirho,
                    name_chirho: "unless".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        bool_ty_chirho.clone(),
                        TyChirho::fun_chirho(io_unit_chirho.clone(), io_unit_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapM_ :: (a -> IO ()) -> [a] -> IO ()
        // mapM_ f []     = return ()
        // mapM_ f (x:xs) = f x >> mapM_ f xs
        {
            let mapm_id_chirho = self.resolve_or_fresh_id_chirho("mapM_");
            let list_a_chirho = TyChirho::ListChirho(Box::new(any_ty_chirho.clone()));
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());

            let h_chirho = self.fresh_binder_chirho("h", any_ty_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_mscr", list_a_chirho.clone());

            // f x >> mapM_ f xs
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
            };
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(mapm_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let then_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "thenIO#".to_string(),
                args_chirho: vec![apply_f_chirho, rec_call_chirho],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                                0,
                            ))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: then_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: mapm_id_chirho,
                    name_chirho: "mapM_".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                        TyChirho::fun_chirho(list_a_chirho.clone(), io_unit_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // forM_ :: [a] -> (a -> IO ()) -> IO ()
        // forM_ xs f = mapM_ f xs
        {
            let form_id_chirho = self.resolve_or_fresh_id_chirho("forM_");
            let list_a_chirho = TyChirho::ListChirho(Box::new(any_ty_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
            );

            let mapm_id_chirho = self.resolve_or_fresh_id_chirho("mapM_");
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(mapm_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: form_id_chirho,
                    name_chirho: "forM_".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho,
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                            io_unit_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // getContents :: IO String — read all stdin as a single String
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("getContents");
            let rhs_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "getContents#".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "getContents".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // putChar :: Char -> IO ()
        {
            let putchar_id_chirho = self.resolve_or_fresh_id_chirho("putChar");
            let c_chirho = self.fresh_binder_chirho("c", TyChirho::char_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putChar#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: putchar_id_chirho,
                    name_chirho: "putChar".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::char_chirho(),
                        io_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        let string_ty_chirho = TyChirho::string_chirho();

        // putStrLn :: String -> IO ()
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("putStrLn");
            let s_chirho = self.fresh_binder_chirho("s", string_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putStrLn#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "putStrLn".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        string_ty_chirho.clone(),
                        io_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // putStr :: String -> IO ()
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("putStr");
            let s_chirho = self.fresh_binder_chirho("s", string_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putStr#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "putStr".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        string_ty_chirho.clone(),
                        io_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // unpack :: String -> [Char]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("unpack");
            let s_chirho = self.fresh_binder_chirho("s", string_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "unpack#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "unpack".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        string_ty_chirho.clone(),
                        string_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // pack :: [Char] -> String
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("pack");
            let s_chirho = self.fresh_binder_chirho("cs", string_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "pack#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "pack".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        string_ty_chirho.clone(),
                        string_ty_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // toUpper :: Char -> Char
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("toUpper");
            let c_chirho = self.fresh_binder_chirho("c", TyChirho::char_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "toUpper#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "toUpper".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::char_chirho(),
                        TyChirho::char_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // toLower :: Char -> Char
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("toLower");
            let c_chirho = self.fresh_binder_chirho("c", TyChirho::char_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "toLower#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "toLower".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::char_chirho(),
                        TyChirho::char_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // isDigit :: Char -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("isDigit");
            let c_chirho = self.fresh_binder_chirho("c", TyChirho::char_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "isDigit#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "isDigit".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::char_chirho(),
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // isAlpha :: Char -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("isAlpha");
            let c_chirho = self.fresh_binder_chirho("c", TyChirho::char_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: c_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "isAlpha#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(c_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "isAlpha".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::char_chirho(),
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // print :: a -> IO ()
        // Simplified: print x = putStrLn (show x)
        // Uses showInt# as default; the dict pass handles type-specific dispatch
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("print");
            let x_chirho = self.fresh_binder_chirho("x", any_ty_chirho.clone());
            let show_primop_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "showInt#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "putStrLn#".to_string(),
                    args_chirho: vec![show_primop_chirho],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "print".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── IORef operations ──

        // newIORef :: a -> IO (IORef a)
        // Simplified: newIORef val = newIORef# val (returns Int id)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("newIORef");
            let v_chirho = self.fresh_binder_chirho("v", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: v_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(v_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "newIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_ty_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // readIORef :: IORef a -> IO a
        // Simplified: readIORef ref = readIORef# ref
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "readIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), any_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // writeIORef :: IORef a -> a -> IO ()
        // Simplified: writeIORef ref val = writeIORef# ref val
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", any_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeIORef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "writeIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(any_ty_chirho.clone(), unit_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // modifyIORef :: IORef a -> (a -> a) -> IO ()
        // Desugared: modifyIORef ref f = writeIORef ref (f (readIORef ref))
        // This avoids needing function application in the primop handler.
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("modifyIORef");
            let read_id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let write_id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_ty_chirho.clone(), any_ty_chirho.clone()),
            );
            // readIORef r
            let read_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(read_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
            };
            // f (readIORef r)
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(read_call_chirho),
            };
            // writeIORef r (f (readIORef r))
            let write_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(write_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(r_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(apply_f_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(write_call_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "modifyIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(
                            TyChirho::fun_chirho(any_ty_chirho.clone(), any_ty_chirho.clone()),
                            unit_ty_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // sequence_ :: [IO ()] -> IO ()
        // sequence_ []     = return ()
        // sequence_ (x:xs) = x >> sequence_ xs
        {
            let seq_id_chirho = self.resolve_or_fresh_id_chirho("sequence_");
            let list_io_chirho = TyChirho::ListChirho(Box::new(io_unit_chirho.clone()));
            let xs_chirho = self.fresh_binder_chirho("xs", list_io_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sscr", list_io_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", io_unit_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_io_chirho.clone());

            // h >> sequence_ t
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(seq_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            let then_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "thenIO#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    rec_call_chirho,
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                                0,
                            ))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: then_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: seq_id_chirho,
                    name_chirho: "sequence_".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_io_chirho, io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // void :: IO a -> IO ()
        // void action = action >> return ()
        {
            let void_id_chirho = self.resolve_or_fresh_id_chirho("void");
            let action_chirho = self.fresh_binder_chirho("action", any_ty_chirho.clone());

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: action_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "thenIO#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(action_chirho.id_chirho),
                        CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                                0,
                            ))],
                        },
                    ],
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: void_id_chirho,
                    name_chirho: "void".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // guard :: Bool -> IO ()
        // guard True  = return ()
        // guard False = error "guard failed"
        {
            let guard_id_chirho = self.resolve_or_fresh_id_chirho("guard");
            let b_chirho = self.fresh_binder_chirho("b", bool_ty_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_gscr", bool_ty_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: io_unit_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "returnIO#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(
                                0,
                            ))],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                CoreLitChirho::StringChirho("guard failed".to_string()),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: b_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: guard_id_chirho,
                    name_chirho: "guard".to_string(),
                    ty_chirho: TyChirho::fun_chirho(bool_ty_chirho.clone(), io_unit_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // interact :: (String -> String) -> IO ()
        // interact f = getContents >>= \input -> putStr (f input)
        {
            let interact_id_chirho = self.resolve_or_fresh_id_chirho("interact");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(string_ty_chirho.clone(), string_ty_chirho.clone()),
            );
            let input_chirho = self.fresh_binder_chirho("input", string_ty_chirho.clone());

            // f input
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(input_chirho.id_chirho)),
            };
            // putStr (f input)  — standard interact uses putStr, not putStrLn
            let putstr_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "putStr#".to_string(),
                args_chirho: vec![apply_f_chirho],
            };
            // \input -> putStr (f input)
            let callback_chirho = CoreExprChirho::LamChirho {
                binder_chirho: input_chirho,
                body_chirho: Box::new(putstr_chirho),
            };
            // getContents >>= \input -> ...
            let bind_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "bindIO#".to_string(),
                args_chirho: vec![
                    CoreExprChirho::PrimOpChirho {
                        name_chirho: "getContents#".to_string(),
                        args_chirho: vec![],
                    },
                    callback_chirho,
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(bind_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: interact_id_chirho,
                    name_chirho: "interact".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(string_ty_chirho.clone(), string_ty_chirho.clone()),
                        io_unit_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Data.Map Prelude functions as thin wrappers over runtime primops.
    ///
    /// Internally the runtime represents a map as `ValueChirho::MapChirho`, a sorted
    /// association list. All heavy lifting (comparison, insertion, lookup, etc.) is
    /// handled by the STG machine via dedicated `PrimOpKindChirho::Map*` variants.
    fn generate_map_prelude_chirho(&mut self) {
        let map_ty_chirho = TyChirho::int_chirho(); // placeholder for Map k v
        let any_k_chirho = TyChirho::VarChirho(TyVarChirho(9980));
        let any_v_chirho = TyChirho::VarChirho(TyVarChirho(9981));
        let any_b_chirho = TyChirho::VarChirho(TyVarChirho(9982));
        let list_kv_ty_chirho = TyChirho::ConChirho("[kv]".to_string());
        let list_k_ty_chirho = TyChirho::ConChirho("[k]".to_string());
        let list_v_ty_chirho = TyChirho::ConChirho("[v]".to_string());
        let bool_ty_chirho = TyChirho::bool_chirho();
        let int_ty_chirho = TyChirho::int_chirho();

        // mapEmpty :: Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapEmpty");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapEmpty".to_string(),
                    ty_chirho: map_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapEmpty#".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapSingleton :: k -> v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapSingleton");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapSingleton#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(any_v_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapSingleton".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapInsert :: k -> v -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapInsert");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapInsert#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(v_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(
                    any_v_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapInsert".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapLookup :: k -> Map k v -> Maybe v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapLookup");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let maybe_ty_chirho = TyChirho::ConChirho("Maybe".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapLookup#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), maybe_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapLookup".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapDelete :: k -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapDelete");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapDelete#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapDelete".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapMember :: k -> Map k v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapMember");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapMember#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), bool_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapMember".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapNotMember :: k -> Map k v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapNotMember");
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "not#".to_string(),
                        args_chirho: vec![CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapMember#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_k_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), bool_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapNotMember".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapSize :: Map k v -> Int
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapSize");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapSize#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), int_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapSize".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapNull :: Map k v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapNull");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapNull#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), bool_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapNull".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFromList :: [(k, v)] -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFromList");
            let xs_chirho = self.fresh_binder_chirho("xs", list_kv_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapFromList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(list_kv_ty_chirho.clone(), map_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFromList".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapToList :: Map k v -> [(k, v)]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapToList");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_kv_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapToList".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapToAscList :: Map k v -> [(k, v)]  (same as toList for sorted impl)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapToAscList");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_kv_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapToAscList".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapKeys :: Map k v -> [k]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapKeys");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapKeys#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_k_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapKeys".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapElems :: Map k v -> [v]
        // Also bind as mapValues (alias)
        for name_chirho in &["mapElems", "mapValues", "mapElemsAsc"] {
            let id_chirho = self.resolve_or_fresh_id_chirho(name_chirho);
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapElems#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), list_v_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: name_chirho.to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapMap :: (a -> b) -> Map k a -> Map k b
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapMap");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapMap#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapMap".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFoldrWithKey :: (k -> v -> b -> b) -> b -> Map k v -> b
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFoldrWithKey");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFoldrWithKey#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_b_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFoldrWithKey".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFoldlWithKey :: (b -> k -> v -> b) -> b -> Map k v -> b
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFoldlWithKey");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFoldlWithKey#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_b_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFoldlWithKey".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFoldr :: (v -> b -> b) -> b -> Map k v -> b
        // Implemented as foldrWithKey ignoring the key
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFoldr");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            // wrapper: \k v acc -> f v acc
            let k2_chirho = self.fresh_binder_chirho("k2", any_k_chirho.clone());
            let v2_chirho = self.fresh_binder_chirho("v2", any_v_chirho.clone());
            let acc2_chirho = self.fresh_binder_chirho("acc2", any_b_chirho.clone());
            let inner_f_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k2_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: acc2_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    v2_chirho.id_chirho,
                                )),
                            }),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(acc2_chirho.id_chirho)),
                        }),
                    }),
                }),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFoldrWithKey#".to_string(),
                            args_chirho: vec![
                                inner_f_chirho,
                                CoreExprChirho::VarChirho(z_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_b_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFoldr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapUnion :: Map k v -> Map k v -> Map k v   (left-biased)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapUnion");
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapUnion#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapUnion".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapUnions :: [Map k v] -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapUnions");
            let xs_chirho = self.fresh_binder_chirho("xs", list_kv_ty_chirho.clone());
            // mapUnions = foldl mapUnion mapEmpty
            let union_id_chirho = self.resolve_or_fresh_id_chirho("mapUnion");
            let empty_id_chirho = self.resolve_or_fresh_id_chirho("mapEmpty");
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                self.resolve_or_fresh_id_chirho("foldl"),
                            )),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(empty_id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(list_kv_ty_chirho.clone(), map_ty_chirho.clone());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapUnions".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapDifference :: Map k v -> Map k w -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapDifference");
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapDifference#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapDifference".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapIntersection :: Map k a -> Map k b -> Map k a
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapIntersection");
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapIntersection#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapIntersection".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapInsertWith :: (v -> v -> v) -> k -> v -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapInsertWith");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: v_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: m_chirho.clone(),
                            body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                                name_chirho: "mapInsertWith#".to_string(),
                                args_chirho: vec![
                                    CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(v_chirho.id_chirho),
                                    CoreExprChirho::VarChirho(m_chirho.id_chirho),
                                ],
                            }),
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    any_k_chirho.clone(),
                    TyChirho::fun_chirho(
                        any_v_chirho.clone(),
                        TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                    ),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapInsertWith".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFindWithDefault :: v -> k -> Map k v -> v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFindWithDefault");
            let def_chirho = self.fresh_binder_chirho("def", any_v_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFindWithDefault#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(def_chirho.id_chirho),
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(
                    any_k_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_v_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFindWithDefault".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapAdjust :: (v -> v) -> k -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapAdjust");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", any_k_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapAdjust#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(
                    any_k_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapAdjust".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapUnionWith :: (v -> v -> v) -> Map k v -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapUnionWith");
            let f_chirho = self.fresh_binder_chirho("f", any_b_chirho.clone());
            let m1_chirho = self.fresh_binder_chirho("m1", map_ty_chirho.clone());
            let m2_chirho = self.fresh_binder_chirho("m2", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m1_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m2_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapUnionWith#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(f_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m1_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m2_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_b_chirho.clone(),
                TyChirho::fun_chirho(
                    map_ty_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapUnionWith".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate extended Data.Map functions that delegate to the same runtime primops.
    fn generate_map_extended_chirho(&mut self) {
        let map_ty_chirho = TyChirho::int_chirho();
        let any_v_chirho = TyChirho::VarChirho(TyVarChirho(9981));

        // mapFilter :: (v -> Bool) -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFilter");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapFilter#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFilter".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFilterWithKey :: (k -> v -> Bool) -> Map k v -> Map k v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFilterWithKey");
            let f_chirho = self.fresh_binder_chirho("f", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapFilterWithKey#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFilterWithKey".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate String-keyed Data.Map operations.
    /// Uses same MapEmpty/MapNode constructors but with eqStr#/ltStr# for comparison.
    /// Generate String-keyed Data.Map operations.
    ///
    /// Since the runtime `compare_values_chirho` handles `StringChirho` keys natively,
    /// these are just re-exports / aliases that delegate to the same Map runtime primops.
    fn generate_map_str_prelude_chirho(&mut self) {
        let str_ty_chirho = TyChirho::ConChirho("String".to_string());
        let any_v_chirho =
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9981));
        let map_ty_chirho = TyChirho::int_chirho(); // placeholder for Map String v

        // mapInsertStr :: String -> v -> Map String v -> Map String v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapInsertStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let v_chirho = self.fresh_binder_chirho("v", any_v_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapInsert#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(v_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(
                    any_v_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapInsertStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapLookupStr :: String -> Map String v -> Maybe v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapLookupStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let maybe_ty_chirho = TyChirho::ConChirho("Maybe".to_string());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapLookup#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), maybe_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapLookupStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapDeleteStr :: String -> Map String v -> Map String v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapDeleteStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapDelete#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), map_ty_chirho.clone()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapDeleteStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapMemberStr :: String -> Map String v -> Bool
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapMemberStr");
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: k_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: m_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "mapMember#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(k_chirho.id_chirho),
                            CoreExprChirho::VarChirho(m_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                str_ty_chirho.clone(),
                TyChirho::fun_chirho(map_ty_chirho.clone(), TyChirho::bool_chirho()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapMemberStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapFindWithDefaultStr :: v -> String -> Map String v -> v
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapFindWithDefaultStr");
            let def_chirho = self.fresh_binder_chirho("def", any_v_chirho.clone());
            let k_chirho = self.fresh_binder_chirho("k", str_ty_chirho.clone());
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: def_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: k_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: m_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                            name_chirho: "mapFindWithDefault#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(def_chirho.id_chirho),
                                CoreExprChirho::VarChirho(k_chirho.id_chirho),
                                CoreExprChirho::VarChirho(m_chirho.id_chirho),
                            ],
                        }),
                    }),
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                any_v_chirho.clone(),
                TyChirho::fun_chirho(
                    str_ty_chirho.clone(),
                    TyChirho::fun_chirho(map_ty_chirho.clone(), any_v_chirho.clone()),
                ),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapFindWithDefaultStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapSizeStr :: Map String v -> Int
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapSizeStr");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapSize#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(map_ty_chirho.clone(), TyChirho::int_chirho());
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapSizeStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // mapToListStr :: Map String v -> [(String, v)]
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("mapToListStr");
            let m_chirho = self.fresh_binder_chirho("m", map_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "mapToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(m_chirho.id_chirho)],
                }),
            };
            let ty_chirho = TyChirho::fun_chirho(
                map_ty_chirho.clone(),
                TyChirho::ConChirho("[kv]".to_string()),
            );
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "mapToListStr".to_string(),
                    ty_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Data.Set Prelude bindings using runtime primops.
    ///
    /// Internally the runtime represents a set as `ValueChirho::SetChirho`, a sorted
    /// deduplicated Vec.  All heavy lifting (comparison, insertion, lookup, etc.) is
    /// handled by the STG machine via dedicated `PrimOpKindChirho::Set*` variants.
    fn generate_set_prelude_chirho(&mut self) {
        let set_ty_chirho = TyChirho::int_chirho(); // placeholder for Set a
        let any_e_chirho =
            TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9985));
        let bool_ty_chirho = TyChirho::bool_chirho();
        let int_ty_chirho = TyChirho::int_chirho();
        let list_ty_chirho = TyChirho::ConChirho("[a]".to_string());

        // ─── setEmpty :: Set a ───────────────────────────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setEmpty");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setEmpty".to_string(),
                    ty_chirho: set_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::PrimOpChirho {
                    name_chirho: "setEmpty#".to_string(),
                    args_chirho: vec![],
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setSingleton :: a -> Set a ──────────────────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setSingleton");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "setSingleton#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setSingleton".to_string(),
                    ty_chirho: TyChirho::fun_chirho(any_e_chirho.clone(), set_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setInsert :: Ord a => a -> Set a -> Set a ───────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setInsert");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "setInsert#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::VarChirho(s_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setInsert".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        any_e_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setMember :: Ord a => a -> Set a -> Bool ────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setMember");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "setMember#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::VarChirho(s_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setMember".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        any_e_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), bool_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setDelete :: Ord a => a -> Set a -> Set a ───────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setDelete");
            let x_chirho = self.fresh_binder_chirho("x", any_e_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "setDelete#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::VarChirho(s_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setDelete".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        any_e_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setSize :: Set a -> Int ─────────────────────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setSize");
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "setSize#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setSize".to_string(),
                    ty_chirho: TyChirho::fun_chirho(set_ty_chirho.clone(), int_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setNull :: Set a -> Bool ────────────────────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setNull");
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "setNull#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setNull".to_string(),
                    ty_chirho: TyChirho::fun_chirho(set_ty_chirho.clone(), bool_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setFromList :: Ord a => [a] -> Set a ────────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setFromList");
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "setFromList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(xs_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setFromList".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_ty_chirho.clone(), set_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setToList :: Set a -> [a] ───────────────────────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setToList");
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: s_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "setToList#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(s_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setToList".to_string(),
                    ty_chirho: TyChirho::fun_chirho(set_ty_chirho.clone(), list_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setUnion :: Ord a => Set a -> Set a -> Set a ────────────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setUnion");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "setUnion#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setUnion".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setIntersection :: Ord a => Set a -> Set a -> Set a ─────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setIntersection");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "setIntersection#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setIntersection".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setDifference :: Ord a => Set a -> Set a -> Set a ───────────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setDifference");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "setDifference#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setDifference".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setFilter :: (a -> Bool) -> Set a -> Set a ──────────────────
        // Implemented via setToList → filter → setFromList
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setFilter");
            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");
            let to_list_id_chirho = self.resolve_or_fresh_id_chirho("setToList");
            let from_list_id_chirho = self.resolve_or_fresh_id_chirho("setFromList");
            let pred_chirho = self.fresh_binder_chirho(
                "pred",
                TyChirho::fun_chirho(any_e_chirho.clone(), bool_ty_chirho.clone()),
            );
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            // setFilter pred s = setFromList (filter pred (setToList s))
            let to_list_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(to_list_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
            };
            let filter_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(pred_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(to_list_call_chirho),
            };
            let from_list_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(from_list_id_chirho)),
                arg_chirho: Box::new(filter_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: pred_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(from_list_call_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setFilter".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(any_e_chirho.clone(), bool_ty_chirho.clone()),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setMap :: Ord b => (a -> b) -> Set a -> Set b ───────────────
        // Implemented via setToList → map → setFromList
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setMap");
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            let to_list_id_chirho = self.resolve_or_fresh_id_chirho("setToList");
            let from_list_id_chirho = self.resolve_or_fresh_id_chirho("setFromList");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(any_e_chirho.clone(), any_e_chirho.clone()),
            );
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            // setMap f s = setFromList (map f (setToList s))
            let to_list_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(to_list_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
            };
            let map_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(map_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(to_list_call_chirho),
            };
            let from_list_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(from_list_id_chirho)),
                arg_chirho: Box::new(map_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: s_chirho,
                    body_chirho: Box::new(from_list_call_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setMap".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(any_e_chirho.clone(), any_e_chirho.clone()),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), set_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setFold :: (a -> b -> b) -> b -> Set a -> b ─────────────────
        // Implemented via foldr over setToList
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setFold");
            let foldr_id_chirho = self.resolve_or_fresh_id_chirho("foldr");
            let to_list_id_chirho = self.resolve_or_fresh_id_chirho("setToList");
            let any_b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9986));
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(
                    any_e_chirho.clone(),
                    TyChirho::fun_chirho(any_b_chirho.clone(), any_b_chirho.clone()),
                ),
            );
            let z_chirho = self.fresh_binder_chirho("z", any_b_chirho.clone());
            let s_chirho = self.fresh_binder_chirho("s", set_ty_chirho.clone());
            // setFold f z s = foldr f z (setToList s)
            let to_list_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(to_list_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(s_chirho.id_chirho)),
            };
            let fold_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldr_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(z_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(to_list_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: z_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: s_chirho,
                        body_chirho: Box::new(fold_call_chirho),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setFold".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_chirho(
                                any_e_chirho.clone(),
                                TyChirho::fun_chirho(any_b_chirho.clone(), any_b_chirho.clone()),
                            ),
                            any_b_chirho.clone(),
                            set_ty_chirho.clone(),
                        ],
                        any_b_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setUnionSize :: Set a -> Set a -> Int  (helper for tests) ───
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setUnionSize");
            let union_id_chirho = self.resolve_or_fresh_id_chirho("setUnion");
            let size_id_chirho = self.resolve_or_fresh_id_chirho("setSize");
            let a_chirho = self.fresh_binder_chirho("a", set_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", set_ty_chirho.clone());
            let union_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(union_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
            };
            let size_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(size_id_chirho)),
                arg_chirho: Box::new(union_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(size_call_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setUnionSize".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        set_ty_chirho.clone(),
                        TyChirho::fun_chirho(set_ty_chirho.clone(), int_ty_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ─── setFromListDedup :: [a] -> Set a (alias for setFromList) ────
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("setFromListDedup");
            let from_list_id_chirho = self.resolve_or_fresh_id_chirho("setFromList");
            let xs_chirho = self.fresh_binder_chirho("xs", list_ty_chirho.clone());
            let call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(from_list_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(call_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "setFromListDedup".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_ty_chirho.clone(), set_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        let _ = any_e_chirho; // suppress unused warning if any path removed
    }

    /// Generate instance dictionary bindings for ground instances.
    ///
    /// For each instance with no context (like `Eq Int`, `Num Int`),
    /// generates a top-level binding:
    /// ```text
    /// $fEqInt = $DictEq $prim_Eq_==_Int
    /// $fNumInt = $DictNum $fEqInt $fShowInt $prim_Num_+_Int ...
    /// ```

    /// Generate Data.Maybe Prelude functions:
    ///   catMaybes :: [Maybe a] -> [a]
    ///   mapMaybe  :: (a -> Maybe b) -> [a] -> [b]
    ///   listToMaybe :: [a] -> Maybe a
    ///   maybeToList :: Maybe a -> [a]
    fn generate_maybe_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));

        // ── maybeToList :: Maybe a -> [a] ──
        // maybeToList Nothing  = []
        // maybeToList (Just x) = [x]
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("maybeToList");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            let singleton_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::ConAppChirho {
                        con_name_chirho: "[]".to_string(),
                        args_chirho: vec![],
                    },
                ],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: singleton_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "maybeToList".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── listToMaybe :: [a] -> Maybe a ──
        // listToMaybe []    = Nothing
        // listToMaybe (x:_) = Just x
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("listToMaybe");
            let xs_chirho = self.fresh_binder_chirho("xs", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sl", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let tl_chirho = self.fresh_binder_chirho("tl", a_chirho.clone());

            let nothing_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Nothing".to_string(),
                args_chirho: vec![],
            };

            let just_x_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Just".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nothing_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, tl_chirho],
                        rhs_chirho: just_x_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "listToMaybe".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── catMaybes :: [Maybe a] -> [a] ──
        // catMaybes []              = []
        // catMaybes (Nothing : xs)  = catMaybes xs
        // catMaybes (Just x  : xs)  = x : catMaybes xs
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("catMaybes");
            let xs_chirho = self.fresh_binder_chirho("xs", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sc", a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let tl_chirho = self.fresh_binder_chirho("tl", a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_sm", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            // catMaybes tl  (recursive call on tail)
            let rec_tail_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(tl_chirho.id_chirho)),
            };

            // x : catMaybes tl
            let cons_x_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    rec_tail_chirho.clone(),
                ],
            };

            // case h of { Nothing -> catMaybes tl; Just x -> x : catMaybes tl }
            let maybe_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_tail_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho],
                        rhs_chirho: cons_x_chirho,
                    },
                ],
            };

            // case xs of { [] -> []; (h:tl) -> case h of ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, tl_chirho],
                        rhs_chirho: maybe_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "catMaybes".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── mapMaybe :: (a -> Maybe b) -> [a] -> [b] ──
        // mapMaybe f []     = []
        // mapMaybe f (x:xs) = case f x of
        //   Nothing -> mapMaybe f xs
        //   Just y  -> y : mapMaybe f xs
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("mapMaybe");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
            );
            let xs_chirho = self.fresh_binder_chirho("xs", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sl", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let tl_chirho = self.fresh_binder_chirho("tl", a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_sm", b_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let nil_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };

            // mapMaybe f tl
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(tl_chirho.id_chirho)),
            };

            // y : mapMaybe f tl
            let cons_y_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    rec_chirho.clone(),
                ],
            };

            // f x
            let fx_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };

            // case f x of { Nothing -> rec; Just y -> y : rec }
            let maybe_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(fx_chirho),
                bind_chirho: scr2_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: rec_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![y_chirho],
                        rhs_chirho: cons_y_chirho,
                    },
                ],
            };

            // case xs of { [] -> []; (x:tl) -> case f x of ... }
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: b_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, tl_chirho],
                        rhs_chirho: maybe_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "mapMaybe".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                        TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── fromJust :: Maybe a -> a ──
        // fromJust (Just x) = x
        // fromJust Nothing  = error "fromJust: Nothing"
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("fromJust");
            let m_chirho = self.fresh_binder_chirho("m", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sfj", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(m_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Just".to_string()),
                        binders_chirho: vec![x_chirho.clone()],
                        rhs_chirho: CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("Nothing".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                CoreLitChirho::StringChirho("fromJust: Nothing".to_string()),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: m_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "fromJust".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── swap :: (a, b) -> (b, a) ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("swap");
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sp", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![x_chirho.clone(), y_chirho.clone()],
                    rhs_chirho: CoreExprChirho::ConAppChirho {
                        con_name_chirho: "$tuple2".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(y_chirho.id_chirho),
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        ],
                    },
                }],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(body_chirho),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "swap".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Data.IORef Prelude bindings.
    /// newIORef, readIORef, writeIORef, modifyIORef are all primops handled by
    /// the STG lowerer and runtime, but we need Core IR wrapper bindings.
    fn generate_ioref_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));

        // ── newIORef :: a -> IORef a ──
        // Just a wrapper that passes through to the primop
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("newIORef");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "newIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── readIORef :: IORef a -> a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("readIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readIORef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "readIORef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── writeIORef :: IORef a -> a -> IO () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("writeIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeIORef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "writeIORef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::int_chirho(), a_chirho.clone()],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── modifyIORef :: IORef a -> (a -> a) -> IO () ──
        // Implemented as: readIORef r >>= \v -> writeIORef r (f v)
        // But since modifyIORef primop doesn't do closure application,
        // implement as: let v = readIORef r in writeIORef r (f v)
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("modifyIORef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
            );
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());

            // readIORef# r
            let read_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "readIORef#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
            };

            // f v
            let fv_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(v_chirho.id_chirho)),
            };

            // writeIORef# r (f v)
            let write_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "writeIORef#".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho), fv_chirho],
            };

            // let v = readIORef# r in writeIORef# r (f v)
            let let_body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(v_chirho, read_chirho)],
                body_chirho: Box::new(write_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho,
                    body_chirho: Box::new(let_body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "modifyIORef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::int_chirho(),
                            TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                        ],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Control.Monad.ST Prelude bindings.
    /// newSTRef, readSTRef, writeSTRef, runST are primops handled by
    /// the STG lowerer and runtime, but we need Core IR wrapper bindings.
    fn generate_st_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));

        // ── newSTRef :: a -> ST s (STRef s a) ──
        // Simplified: STRef s a ≈ Int (same as IORef), ST s ≈ identity
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("newSTRef");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newSTRef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "newSTRef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── readSTRef :: STRef s a -> ST s a ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("readSTRef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readSTRef#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "readSTRef".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── writeSTRef :: STRef s a -> a -> ST s () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("writeSTRef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeSTRef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "writeSTRef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::int_chirho(), a_chirho.clone()],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── modifySTRef :: STRef s a -> (a -> a) -> ST s () ──
        // Simplified: STRef s a ≈ Int, applies f to stored value and writes back
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("modifySTRef");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
            );
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: f_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "modifySTRef#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(f_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "modifySTRef".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::int_chirho(),
                            TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                        ],
                        TyChirho::unit_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── runST :: (forall s. ST s a) -> a ──
        // Simplified: runST f = f (just evaluate the ST computation)
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("runST");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "runST#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(f_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "runST".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── STM (Software Transactional Memory) operations ──

        // newTVar :: a -> STM (TVar a)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("newTVar");
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: v_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newTVar#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(v_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "newTVar".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // newTVarIO :: a -> IO (TVar a)  (convenience for creating TVars in IO)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("newTVarIO");
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: v_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "newTVar#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(v_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "newTVarIO".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // readTVar :: TVar a -> STM a
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("readTVar");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readTVar#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "readTVar".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // readTVarIO :: TVar a -> IO a  (convenience for reading TVars in IO)
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("readTVarIO");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "readTVar#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(r_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "readTVarIO".to_string(),
                    ty_chirho: TyChirho::fun_chirho(TyChirho::int_chirho(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // writeTVar :: TVar a -> a -> STM ()
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("writeTVar");
            let r_chirho = self.fresh_binder_chirho("r", TyChirho::int_chirho());
            let v_chirho = self.fresh_binder_chirho("v", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: r_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: v_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "writeTVar#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(r_chirho.id_chirho),
                            CoreExprChirho::VarChirho(v_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "writeTVar".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(a_chirho.clone(), TyChirho::int_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // atomically :: STM a -> IO a
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("atomically");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "atomically#".to_string(),
                    args_chirho: vec![CoreExprChirho::VarChirho(f_chirho.id_chirho)],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "atomically".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // retry :: STM a
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("retry");
            let rhs_chirho = CoreExprChirho::PrimOpChirho {
                name_chirho: "retry#".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "retry".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // orElse :: STM a -> STM a -> STM a
        {
            let id_chirho = self.resolve_or_fresh_id_chirho("orElse");
            let a1_chirho = self.fresh_binder_chirho("a1", a_chirho.clone());
            let a2_chirho = self.fresh_binder_chirho("a2", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a1_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: a2_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "orElse#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a1_chirho.id_chirho),
                            CoreExprChirho::VarChirho(a2_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho,
                    name_chirho: "orElse".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate higher-order list functions: sortBy, nubBy, maximumBy,
    /// minimumBy, groupBy, on, zipWith, and related.
    fn generate_higher_order_list_prelude_chirho(&mut self) {
        let a_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(9991));
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };
        let cons_chirho =
            |hd_chirho: CoreExprChirho, tl_chirho: CoreExprChirho| CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![hd_chirho, tl_chirho],
            };

        // ── sortBy :: (a -> a -> Ordering) -> [a] -> [a] ──
        // Insertion sort using the comparison function:
        // sortBy cmp [] = []
        // sortBy cmp (x:xs) = insertBy cmp x (sortBy cmp xs)
        // insertBy cmp x [] = [x]
        // insertBy cmp x (y:ys) = case cmp x y of
        //   GT -> y : insertBy cmp x ys
        //   _  -> x : y : ys
        {
            let sortby_id_chirho = self.resolve_or_fresh_id_chirho("sortBy");
            let insertby_id_chirho = self.resolve_or_fresh_id_chirho("insertBy");

            // insertBy cmp x ys
            let cmp_ib_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let x_ib_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let ys_ib_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let scr_ib_chirho = self.fresh_binder_chirho("_ib", list_a_chirho.clone());
            let y_ib_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let rest_ib_chirho = self.fresh_binder_chirho("rest", list_a_chirho.clone());

            // cmp x y → case result of GT → y : insertBy cmp x rest; _ → x : y : rest
            let cmp_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(cmp_ib_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_ib_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_ib_chirho.id_chirho)),
            };
            let scr_cmp_chirho = self.fresh_binder_chirho("_cmpres", a_chirho.clone());

            let rec_insert_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(insertby_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(cmp_ib_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_ib_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_ib_chirho.id_chirho)),
            };

            let gt_rhs_chirho = cons_chirho(
                CoreExprChirho::VarChirho(y_ib_chirho.id_chirho),
                rec_insert_chirho,
            );
            let default_rhs_chirho = cons_chirho(
                CoreExprChirho::VarChirho(x_ib_chirho.id_chirho),
                cons_chirho(
                    CoreExprChirho::VarChirho(y_ib_chirho.id_chirho),
                    CoreExprChirho::VarChirho(rest_ib_chirho.id_chirho),
                ),
            );

            let cmp_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cmp_call_chirho),
                bind_chirho: scr_cmp_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: gt_rhs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: default_rhs_chirho,
                    },
                ],
            };

            let cons_alt_chirho = CoreAltChirho {
                con_chirho: AltConChirho::DataConChirho(":".to_string()),
                binders_chirho: vec![y_ib_chirho.clone(), rest_ib_chirho.clone()],
                rhs_chirho: cmp_case_chirho,
            };

            let insertby_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(ys_ib_chirho.id_chirho)),
                bind_chirho: scr_ib_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: cons_chirho(
                            CoreExprChirho::VarChirho(x_ib_chirho.id_chirho),
                            nil_chirho(),
                        ),
                    },
                    cons_alt_chirho,
                ],
            };

            let insertby_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_ib_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: x_ib_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: ys_ib_chirho,
                        body_chirho: Box::new(insertby_body_chirho),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: insertby_id_chirho,
                    name_chirho: "insertBy".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: insertby_rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            // sortBy cmp [] = []
            // sortBy cmp (x:xs) = insertBy cmp x (sortBy cmp xs)
            let cmp_sb_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let xs_sb_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_sb_chirho = self.fresh_binder_chirho("_sb", list_a_chirho.clone());
            let h_sb_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_sb_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());

            let rec_sort_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(sortby_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(cmp_sb_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_sb_chirho.id_chirho)),
            };

            let insert_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(insertby_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(cmp_sb_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_sb_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_sort_chirho),
            };

            let sortby_body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_sb_chirho.id_chirho)),
                bind_chirho: scr_sb_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_sb_chirho.clone(), t_sb_chirho.clone()],
                        rhs_chirho: insert_call_chirho,
                    },
                ],
            };

            let sortby_rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_sb_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_sb_chirho,
                    body_chirho: Box::new(sortby_body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: sortby_id_chirho,
                    name_chirho: "sortBy".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: sortby_rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── nubBy :: (a -> a -> Bool) -> [a] -> [a] ──
        // nubBy eq [] = []
        // nubBy eq (x:xs) = x : nubBy eq (filter (\y -> not (eq x y)) xs)
        {
            let nubby_id_chirho = self.resolve_or_fresh_id_chirho("nubBy");
            let filter_id_chirho = self.resolve_or_fresh_id_chirho("filter");
            let not_id_chirho = self.resolve_or_fresh_id_chirho("not");

            let eq_chirho = self.fresh_binder_chirho("eq", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_nb", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());

            // \y -> not (eq h y)
            let eq_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(eq_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let not_eq_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(not_id_chirho)),
                arg_chirho: Box::new(eq_call_chirho),
            };
            let pred_lambda_chirho = CoreExprChirho::LamChirho {
                binder_chirho: y_chirho,
                body_chirho: Box::new(not_eq_chirho),
            };

            // filter pred t
            let filtered_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(filter_id_chirho)),
                    arg_chirho: Box::new(pred_lambda_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // nubBy eq (filtered)
            let rec_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(nubby_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(eq_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(filtered_chirho),
            };

            let cons_rhs_chirho =
                cons_chirho(CoreExprChirho::VarChirho(h_chirho.id_chirho), rec_chirho);

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho.clone(), t_chirho.clone()],
                        rhs_chirho: cons_rhs_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: eq_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: nubby_id_chirho,
                    name_chirho: "nubBy".to_string(),
                    ty_chirho: list_a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── maximumBy :: (a -> a -> Ordering) -> [a] -> a ──
        // maximumBy cmp (x:xs) = foldl (\acc y -> case cmp acc y of { LT -> y; _ -> acc }) x xs
        {
            let maxby_id_chirho = self.resolve_or_fresh_id_chirho("maximumBy");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");

            let cmp_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_mx", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let acc_chirho = self.fresh_binder_chirho("acc", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let scr_cmp_chirho = self.fresh_binder_chirho("_mc", a_chirho.clone());

            // \acc y -> case cmp acc y of { LT -> y; _ -> acc }
            let cmp_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(cmp_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(acc_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cmp_call_chirho),
                bind_chirho: scr_cmp_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(acc_chirho.id_chirho),
                    },
                ],
            };
            let fold_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: acc_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho,
                    body_chirho: Box::new(case_chirho),
                }),
            };

            // case xs of { (h:t) -> foldl fn h t }
            let foldl_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(fold_fn_chirho),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho.clone(), t_chirho.clone()],
                        rhs_chirho: foldl_call_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                crate::expr_chirho::CoreLitChirho::StringChirho(
                                    "maximumBy: empty list".to_string(),
                                ),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: maxby_id_chirho,
                    name_chirho: "maximumBy".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── minimumBy :: (a -> a -> Ordering) -> [a] -> a ──
        // Same as maximumBy but with GT instead of LT
        {
            let minby_id_chirho = self.resolve_or_fresh_id_chirho("minimumBy");
            let foldl_id_chirho = self.resolve_or_fresh_id_chirho("foldl");

            let cmp_chirho = self.fresh_binder_chirho("cmp", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_mn", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let acc_chirho = self.fresh_binder_chirho("acc", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let scr_cmp_chirho = self.fresh_binder_chirho("_mc", a_chirho.clone());

            let cmp_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(cmp_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(acc_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(cmp_call_chirho),
                bind_chirho: scr_cmp_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(y_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(acc_chirho.id_chirho),
                    },
                ],
            };
            let fold_fn_chirho = CoreExprChirho::LamChirho {
                binder_chirho: acc_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho,
                    body_chirho: Box::new(case_chirho),
                }),
            };

            let foldl_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(foldl_id_chirho)),
                        arg_chirho: Box::new(fold_fn_chirho),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(h_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho.clone(), t_chirho.clone()],
                        rhs_chirho: foldl_call_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "error#".to_string(),
                            args_chirho: vec![CoreExprChirho::LitChirho(
                                crate::expr_chirho::CoreLitChirho::StringChirho(
                                    "minimumBy: empty list".to_string(),
                                ),
                            )],
                        },
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: cmp_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: minby_id_chirho,
                    name_chirho: "minimumBy".to_string(),
                    ty_chirho: a_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── on :: (b -> b -> c) -> (a -> b) -> a -> a -> c ──
        // on f g x y = f (g x) (g y)
        {
            let on_id_chirho = self.resolve_or_fresh_id_chirho("on");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let g_chirho = self.fresh_binder_chirho("g", a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", a_chirho.clone());

            let gx_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(g_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let gy_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(g_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                    arg_chirho: Box::new(gx_chirho),
                }),
                arg_chirho: Box::new(gy_chirho),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: g_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: x_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: y_chirho,
                            body_chirho: Box::new(body_chirho),
                        }),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: on_id_chirho,
                    name_chirho: "on".to_string(),
                    ty_chirho: b_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── find :: (a -> Bool) -> [a] -> Maybe a ──
        // find _ [] = Nothing
        // find p (x:xs) = if p x then Just x else find p xs
        {
            let find_id_chirho = self.resolve_or_fresh_id_chirho("find");

            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_fscr", list_a_chirho.clone());

            // Nothing constructor
            let nothing_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Nothing".to_string(),
                args_chirho: vec![],
            };

            // Just x
            let just_x_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "Just".to_string(),
                args_chirho: vec![CoreExprChirho::VarChirho(x_chirho.id_chirho)],
            };

            // find p rest
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(find_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };

            // p x
            let p_x_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };

            let maybe_a_chirho = TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                Box::new(a_chirho.clone()),
            );

            // if p x then Just x else find p rest
            let scr_p_chirho = self.fresh_binder_chirho("_px", TyChirho::bool_chirho());
            let if_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(p_x_chirho),
                bind_chirho: scr_p_chirho,
                result_ty_chirho: maybe_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: just_x_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DefaultChirho,
                        binders_chirho: vec![],
                        rhs_chirho: rec_call_chirho,
                    },
                ],
            };

            // case xs of [] -> Nothing; (x:rest) -> if p x then Just x else find p rest
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: maybe_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nothing_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: if_case_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: p_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: find_id_chirho,
                    name_chirho: "find".to_string(),
                    ty_chirho: maybe_a_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── groupBy :: (a -> a -> Bool) -> [a] -> [[a]] ──
        // groupBy _ [] = []
        // groupBy eq (x:xs) = let (ys, zs) = span (eq x) xs
        //                     in (x:ys) : groupBy eq zs
        {
            let groupby_id_chirho = self.resolve_or_fresh_id_chirho("groupBy");
            let span_id_chirho = self.resolve_or_fresh_id_chirho("span");

            let eq_chirho = self.fresh_binder_chirho("eq", a_chirho.clone());
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_gbscr", list_a_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());

            let list_list_a_chirho = TyChirho::ListChirho(Box::new(list_a_chirho.clone()));

            // \y -> eq x y  (the predicate for span)
            let y_lam_chirho = self.fresh_binder_chirho("y", a_chirho.clone());
            let eq_x_y_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(eq_chirho.id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(y_lam_chirho.id_chirho)),
            };
            let pred_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: y_lam_chirho,
                body_chirho: Box::new(eq_x_y_chirho),
            };

            // span (\y -> eq x y) rest
            let span_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(pred_lam_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };

            // groupBy eq zs
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(groupby_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(eq_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(zs_chirho.id_chirho)),
            };

            // (x:ys) : groupBy eq zs
            let result_chirho = cons_chirho(
                cons_chirho(
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                ),
                rec_call_chirho,
            );

            // case pair of ($tuple2 ys zs) -> result
            let case_pair_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_gbp", a_chirho.clone()),
                result_ty_chirho: list_list_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![ys_chirho, zs_chirho],
                    rhs_chirho: result_chirho,
                }],
            };

            // let pair = span (\y -> eq x y) rest in case pair of ...
            let let_span_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(pair_chirho, span_call_chirho)],
                body_chirho: Box::new(case_pair_chirho),
            };

            // case xs of [] -> []; (x:rest) -> let_span
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: let_span_chirho,
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: eq_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: groupby_id_chirho,
                    name_chirho: "groupBy".to_string(),
                    ty_chirho: list_list_a_chirho,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate Semigroup and Monoid ground instance bindings.
    /// Semigroup [a] / [Char]: `<>` = `++#`  (list/string append)
    /// Monoid [a] / [Char]: `mempty` = `[]`
    fn generate_semigroup_monoid_prelude_chirho(&mut self) {
        // ── $prim_Semigroup_<>_[Char] : wraps ++# (string append) ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_[Char]");
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::string_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "++#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_[Char]".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── $prim_Semigroup_<>_[Int] etc.: delegates to `append` (list-level) ──
        let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
        for type_key_chirho in &["[Int]", "[Integer]", "[Double]", "[Bool]"] {
            let fn_id_chirho =
                self.resolve_or_fresh_id_chirho(&format!("$prim_Semigroup_<>_{}", type_key_chirho));
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::string_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(b_chirho.id_chirho)),
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: format!("$prim_Semigroup_<>_{}", type_key_chirho),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── $prim_Monoid_mempty_[Int] etc.: returns [] ──
        for type_key_chirho in &["[Int]", "[Integer]", "[Char]", "[Double]", "[Bool]"] {
            let fn_id_chirho = self
                .resolve_or_fresh_id_chirho(&format!("$prim_Monoid_mempty_{}", type_key_chirho));
            let rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "[]".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: format!("$prim_Monoid_mempty_{}", type_key_chirho),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Data.Monoid.Sum: interface newtype is erased to its payload ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_Sum");
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::int_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "+#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_Sum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mempty_Sum");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mempty_Sum".to_string(),
                    ty_chirho: TyChirho::int_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mconcat_Sum");
            let xs_chirho = self
                .fresh_binder_chirho("xs", TyChirho::ListChirho(Box::new(TyChirho::int_chirho())));
            let x_chirho = self.fresh_binder_chirho("x", TyChirho::int_chirho());
            let rest_chirho = self.fresh_binder_chirho(
                "rest",
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            );
            let wild_chirho = self.fresh_binder_chirho("wild", TyChirho::int_chirho());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "+#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                                rec_call_chirho,
                            ],
                        },
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mconcat_Sum".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                        TyChirho::int_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Data.Monoid.Product: interface newtype is erased to its payload ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_Product");
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::int_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::int_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "*#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(a_chirho.id_chirho),
                            CoreExprChirho::VarChirho(b_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_Product".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mempty_Product");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mempty_Product".to_string(),
                    ty_chirho: TyChirho::int_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mconcat_Product");
            let xs_chirho = self
                .fresh_binder_chirho("xs", TyChirho::ListChirho(Box::new(TyChirho::int_chirho())));
            let x_chirho = self.fresh_binder_chirho("x", TyChirho::int_chirho());
            let rest_chirho = self.fresh_binder_chirho(
                "rest",
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            );
            let wild_chirho = self.fresh_binder_chirho("wild", TyChirho::int_chirho());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::int_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(1)),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: CoreExprChirho::PrimOpChirho {
                            name_chirho: "*#".to_string(),
                            args_chirho: vec![
                                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                                rec_call_chirho,
                            ],
                        },
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mconcat_Product".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                        TyChirho::int_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        fn bool_con_chirho(con_name_chirho: &str) -> CoreExprChirho {
            CoreExprChirho::ConAppChirho {
                con_name_chirho: con_name_chirho.to_string(),
                args_chirho: vec![],
            }
        }

        for (type_key_chirho, empty_con_chirho) in [("All", "True"), ("Any", "False")] {
            let semigroup_name_chirho = format!("$prim_Semigroup_<>_{}", type_key_chirho);
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(&semigroup_name_chirho);
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::bool_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::bool_chirho());
            let wild_chirho = self.fresh_binder_chirho("wild", TyChirho::bool_chirho());
            let (true_rhs_chirho, false_rhs_chirho) = if type_key_chirho == "All" {
                (
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                    bool_con_chirho("False"),
                )
            } else {
                (
                    bool_con_chirho("True"),
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                )
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: true_rhs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: false_rhs_chirho,
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: semigroup_name_chirho,
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::bool_chirho(),
                        TyChirho::fun_chirho(TyChirho::bool_chirho(), TyChirho::bool_chirho()),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            let mempty_name_chirho = format!("$prim_Monoid_mempty_{}", type_key_chirho);
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(&mempty_name_chirho);
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: mempty_name_chirho,
                    ty_chirho: TyChirho::bool_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: bool_con_chirho(empty_con_chirho),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });

            let mconcat_name_chirho = format!("$prim_Monoid_mconcat_{}", type_key_chirho);
            let fn_id_chirho = self.resolve_or_fresh_id_chirho(&mconcat_name_chirho);
            let xs_chirho = self.fresh_binder_chirho(
                "xs",
                TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
            );
            let x_chirho = self.fresh_binder_chirho("x", TyChirho::bool_chirho());
            let rest_chirho = self.fresh_binder_chirho(
                "rest",
                TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
            );
            let list_wild_chirho = self.fresh_binder_chirho("wild", TyChirho::bool_chirho());
            let bool_wild_chirho = self.fresh_binder_chirho("wild", TyChirho::bool_chirho());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let (true_rhs_chirho, false_rhs_chirho) = if type_key_chirho == "All" {
                (rec_call_chirho, bool_con_chirho("False"))
            } else {
                (bool_con_chirho("True"), rec_call_chirho)
            };
            let combine_case_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                bind_chirho: bool_wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("True".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: true_rhs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("False".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: false_rhs_chirho,
                    },
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: list_wild_chirho,
                result_ty_chirho: TyChirho::bool_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: bool_con_chirho(empty_con_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: combine_case_chirho,
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: mconcat_name_chirho,
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
                        TyChirho::bool_chirho(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        fn ordering_con_chirho(con_name_chirho: &str) -> CoreExprChirho {
            CoreExprChirho::ConAppChirho {
                con_name_chirho: con_name_chirho.to_string(),
                args_chirho: vec![],
            }
        }

        let ordering_ty_chirho = TyChirho::ConChirho("Ordering".to_string());

        // ── Ordering: left-biased Semigroup, EQ as Monoid identity ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_Ordering");
            let a_chirho = self.fresh_binder_chirho("a", ordering_ty_chirho.clone());
            let b_chirho = self.fresh_binder_chirho("b", ordering_ty_chirho.clone());
            let wild_chirho = self.fresh_binder_chirho("wild", ordering_ty_chirho.clone());
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(a_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: ordering_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("LT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: ordering_con_chirho("LT"),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("EQ".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::VarChirho(b_chirho.id_chirho),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("GT".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: ordering_con_chirho("GT"),
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_Ordering".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        ordering_ty_chirho.clone(),
                        TyChirho::fun_chirho(
                            ordering_ty_chirho.clone(),
                            ordering_ty_chirho.clone(),
                        ),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: a_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: b_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                },
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mempty_Ordering");
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mempty_Ordering".to_string(),
                    ty_chirho: ordering_ty_chirho.clone(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: ordering_con_chirho("EQ"),
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mconcat_Ordering");
            let semigroup_id_chirho =
                self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_Ordering");
            let xs_chirho = self.fresh_binder_chirho(
                "xs",
                TyChirho::ListChirho(Box::new(ordering_ty_chirho.clone())),
            );
            let x_chirho = self.fresh_binder_chirho("x", ordering_ty_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho(
                "rest",
                TyChirho::ListChirho(Box::new(ordering_ty_chirho.clone())),
            );
            let wild_chirho = self.fresh_binder_chirho("wild", ordering_ty_chirho.clone());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };
            let append_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(semigroup_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_call_chirho),
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: ordering_ty_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: ordering_con_chirho("EQ"),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), rest_chirho.clone()],
                        rhs_chirho: append_chirho,
                    },
                ],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mconcat_Ordering".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(ordering_ty_chirho.clone())),
                        ordering_ty_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho: CoreExprChirho::LamChirho {
                    binder_chirho: xs_chirho,
                    body_chirho: Box::new(body_chirho),
                },
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── Prelude-level <> alias ──
        // <> is a class method, goes through dict pass. The dict pass
        // should handle rewriting references to <> via the selector +
        // dictionary application. The $prim bindings above provide
        // the implementations for ground instances.

        // ── Prelude-level mempty alias ──
        // Same as <> — goes through dict pass.

        // ── $prim_Semigroup_<>_() : \a b -> () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Semigroup_<>_()");
            let a_chirho = self.fresh_binder_chirho("a", TyChirho::string_chirho());
            let b_chirho = self.fresh_binder_chirho("b", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: a_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: b_chirho,
                    body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                        con_name_chirho: "()".to_string(),
                        args_chirho: vec![],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Semigroup_<>_()".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── $prim_Monoid_mempty_() : () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mempty_()");
            let rhs_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "()".to_string(),
                args_chirho: vec![],
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mempty_()".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── $prim_Monoid_mconcat_() : \xs -> () ──
        {
            let fn_id_chirho = self.resolve_or_fresh_id_chirho("$prim_Monoid_mconcat_()");
            let xs_chirho = self.fresh_binder_chirho("xs", TyChirho::string_chirho());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(CoreExprChirho::ConAppChirho {
                    con_name_chirho: "()".to_string(),
                    args_chirho: vec![],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: "$prim_Monoid_mconcat_()".to_string(),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── mconcat :: [a] -> a  (foldr (<>) mempty) ──
        // For [Char] (strings): uses ++# primop.
        // For other list types: uses append function.
        let mconcat_append_id_chirho = self.resolve_or_fresh_id_chirho("append");
        for type_key_chirho in &["[Int]", "[Integer]", "[Char]", "[Double]", "[Bool]"] {
            let fn_id_chirho = self
                .resolve_or_fresh_id_chirho(&format!("$prim_Monoid_mconcat_{}", type_key_chirho));
            // mconcat xss = case xss of { [] -> []; (x:xs) -> x <> mconcat xs }
            let xss_chirho = self.fresh_binder_chirho("xss", TyChirho::string_chirho());
            let x_chirho = self.fresh_binder_chirho("x", TyChirho::string_chirho());
            let xs_chirho = self.fresh_binder_chirho("xs", TyChirho::string_chirho());
            let wild_chirho = self.fresh_binder_chirho("wild", TyChirho::string_chirho());
            // Recursive call: mconcat xs
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(fn_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            // x <> (mconcat xs) — use ++# for [Char], append for others
            let append_chirho = if *type_key_chirho == "[Char]" {
                CoreExprChirho::PrimOpChirho {
                    name_chirho: "++#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        rec_call_chirho,
                    ],
                }
            } else {
                CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(mconcat_append_id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(rec_call_chirho),
                }
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xss_chirho.id_chirho)),
                bind_chirho: wild_chirho,
                result_ty_chirho: TyChirho::string_chirho(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: CoreExprChirho::ConAppChirho {
                            con_name_chirho: "[]".to_string(),
                            args_chirho: vec![],
                        },
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho.clone(), xs_chirho.clone()],
                        rhs_chirho: append_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xss_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fn_id_chirho,
                    name_chirho: format!("$prim_Monoid_mconcat_{}", type_key_chirho),
                    ty_chirho: TyChirho::string_chirho(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate a ground `Class [ElemType]` dict with working method implementations.

    /// Generate additional utility Prelude functions:
    /// repeat, cycle, group, groupBy, transpose, fix, zipWith3, and Data.Tuple extras.
    fn generate_utility_prelude_chirho(&mut self) {
        use haskelujah_typing_chirho::ty_chirho::TyVarChirho;

        let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let b_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let c_chirho = TyChirho::VarChirho(TyVarChirho(9992));
        let d_chirho = TyChirho::VarChirho(TyVarChirho(9993));
        let int_chirho = TyChirho::int_chirho();
        let _bool_chirho = TyChirho::bool_chirho();
        let list_a_chirho = TyChirho::ListChirho(Box::new(a_chirho.clone()));
        let list_b_chirho = TyChirho::ListChirho(Box::new(b_chirho.clone()));
        let list_c_chirho = TyChirho::ListChirho(Box::new(c_chirho.clone()));

        let nil_chirho = || CoreExprChirho::ConAppChirho {
            con_name_chirho: "[]".to_string(),
            args_chirho: vec![],
        };
        let cons_chirho =
            |hd_chirho: CoreExprChirho, tl_chirho: CoreExprChirho| CoreExprChirho::ConAppChirho {
                con_name_chirho: ":".to_string(),
                args_chirho: vec![hd_chirho, tl_chirho],
            };

        // ── repeat :: a -> [a] ──
        // repeat x = x : repeat x
        {
            let repeat_id_chirho = self.resolve_or_fresh_id_chirho("repeat");
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(repeat_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let body_chirho = cons_chirho(
                CoreExprChirho::VarChirho(x_chirho.id_chirho),
                rec_call_chirho,
            );
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: repeat_id_chirho,
                    name_chirho: "repeat".to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── cycle :: [a] -> [a] ──
        // cycle xs = xs ++ cycle xs
        // (Uses Prelude append)
        {
            let cycle_id_chirho = self.resolve_or_fresh_id_chirho("cycle");
            let append_id_chirho = self.resolve_or_fresh_id_chirho("append");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(cycle_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(append_id_chirho)),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(rec_call_chirho),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: cycle_id_chirho,
                    name_chirho: "cycle".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── zipWith3 :: (a -> b -> c -> d) -> [a] -> [b] -> [c] -> [d] ──
        // zipWith3 f (a:as) (b:bs) (c:cs) = f a b c : zipWith3 f as bs cs
        // zipWith3 _ _ _ _ = []
        {
            let zw3_id_chirho = self.resolve_or_fresh_id_chirho("zipWith3");
            let f_chirho = self.fresh_binder_chirho("f", a_chirho.clone());
            let as_chirho = self.fresh_binder_chirho("as_", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let cs_chirho = self.fresh_binder_chirho("cs", list_c_chirho.clone());
            let a1_chirho = self.fresh_binder_chirho("a1", a_chirho.clone());
            let at_chirho = self.fresh_binder_chirho("at", list_a_chirho.clone());
            let b1_chirho = self.fresh_binder_chirho("b1", b_chirho.clone());
            let bt_chirho = self.fresh_binder_chirho("bt", list_b_chirho.clone());
            let c1_chirho = self.fresh_binder_chirho("c1", c_chirho.clone());
            let ct_chirho = self.fresh_binder_chirho("ct", list_c_chirho.clone());
            let scr1_chirho = self.fresh_binder_chirho("_zscr1", list_a_chirho.clone());
            let scr2_chirho = self.fresh_binder_chirho("_zscr2", list_b_chirho.clone());
            let scr3_chirho = self.fresh_binder_chirho("_zscr3", list_c_chirho.clone());

            // f a1 b1 c1
            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(a1_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(b1_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(c1_chirho.id_chirho)),
            };
            // rec call
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(zw3_id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(at_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(bt_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(ct_chirho.id_chirho)),
            };
            let cons_result_chirho = cons_chirho(apply_f_chirho, rec_call_chirho);

            // case cs of (c1:ct) -> cons_result; [] -> []
            let case_cs_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(cs_chirho.id_chirho)),
                bind_chirho: scr3_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![c1_chirho, ct_chirho],
                        rhs_chirho: cons_result_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };
            // case bs of (b1:bt) -> case_cs; [] -> []
            let case_bs_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                bind_chirho: scr2_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![b1_chirho, bt_chirho],
                        rhs_chirho: case_cs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };
            // case as_ of (a1:at) -> case_bs; [] -> []
            let case_as_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                bind_chirho: scr1_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![a1_chirho, at_chirho],
                        rhs_chirho: case_bs_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                ],
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: as_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: bs_chirho,
                        body_chirho: Box::new(CoreExprChirho::LamChirho {
                            binder_chirho: cs_chirho,
                            body_chirho: Box::new(case_as_chirho),
                        }),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zw3_id_chirho,
                    name_chirho: "zipWith3".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::fun_n_chirho(
                                vec![a_chirho.clone(), b_chirho.clone(), c_chirho.clone()],
                                d_chirho.clone(),
                            ),
                            list_a_chirho.clone(),
                            list_b_chirho.clone(),
                            list_c_chirho.clone(),
                        ],
                        TyChirho::ListChirho(Box::new(d_chirho.clone())),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── zip3 :: [a] -> [b] -> [c] -> [(a, b, c)] ──
        // zip3 = zipWith3 (\a b c -> (a, b, c))
        {
            let zip3_id_chirho = self.resolve_or_fresh_id_chirho("zip3");
            let zw3_id_chirho = self.resolve_or_fresh_id_chirho("zipWith3");
            let as_chirho = self.fresh_binder_chirho("as_", list_a_chirho.clone());
            let bs_chirho = self.fresh_binder_chirho("bs", list_b_chirho.clone());
            let cs_chirho = self.fresh_binder_chirho("cs", list_c_chirho.clone());
            let la_chirho = self.fresh_binder_chirho("a", a_chirho.clone());
            let lb_chirho = self.fresh_binder_chirho("b", b_chirho.clone());
            let lc_chirho = self.fresh_binder_chirho("c", c_chirho.clone());

            // \a b c -> (a, b, c) — represented as $tuple3 a b c
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple3".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(la_chirho.id_chirho),
                    CoreExprChirho::VarChirho(lb_chirho.id_chirho),
                    CoreExprChirho::VarChirho(lc_chirho.id_chirho),
                ],
            };
            let tuple_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: la_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: lb_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: lc_chirho,
                        body_chirho: Box::new(mk_tuple_chirho),
                    }),
                }),
            };

            // zipWith3 tuple_lam as bs cs
            let body_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::AppChirho {
                        fun_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(zw3_id_chirho)),
                            arg_chirho: Box::new(tuple_lam_chirho),
                        }),
                        arg_chirho: Box::new(CoreExprChirho::VarChirho(as_chirho.id_chirho)),
                    }),
                    arg_chirho: Box::new(CoreExprChirho::VarChirho(bs_chirho.id_chirho)),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(cs_chirho.id_chirho)),
            };

            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: as_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: bs_chirho,
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: cs_chirho,
                        body_chirho: Box::new(body_chirho),
                    }),
                }),
            };

            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: zip3_id_chirho,
                    name_chirho: "zip3".to_string(),
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![
                            list_a_chirho.clone(),
                            list_b_chirho.clone(),
                            list_c_chirho.clone(),
                        ],
                        list_a_chirho.clone(), // placeholder
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── fix :: (a -> a) -> a ──
        // fix f = let x = f x in x
        {
            let fix_id_chirho = self.resolve_or_fresh_id_chirho("fix");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
            );
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());

            let f_x_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let body_chirho = CoreExprChirho::LetChirho {
                rec_chirho: true,
                binds_chirho: vec![(x_chirho.clone(), f_x_chirho)],
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: fix_id_chirho,
                    name_chirho: "fix".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho.clone()),
                        a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── group :: [Int] -> [[Int]] ──
        // group [] = []
        // group (x:xs) = let (ys, zs) = span (==# x) xs
        //                in (x:ys) : group zs
        {
            let group_id_chirho = self.resolve_or_fresh_id_chirho("group");
            let span_id_chirho = self.resolve_or_fresh_id_chirho("span");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let x_chirho = self.fresh_binder_chirho("x", int_chirho.clone());
            let rest_chirho = self.fresh_binder_chirho("rest", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_gscr", list_a_chirho.clone());
            let pair_chirho = self.fresh_binder_chirho("pair", a_chirho.clone());
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let zs_chirho = self.fresh_binder_chirho("zs", list_a_chirho.clone());

            // \e -> e ==# x
            let eq_lam_param_chirho = self.fresh_binder_chirho("e", int_chirho.clone());
            let eq_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: eq_lam_param_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "==#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(eq_lam_param_chirho.id_chirho),
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    ],
                }),
            };

            // span (\e -> e ==# x) rest
            let span_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(span_id_chirho)),
                    arg_chirho: Box::new(eq_lam_chirho),
                }),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(rest_chirho.id_chirho)),
            };

            // group zs
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(group_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(zs_chirho.id_chirho)),
            };

            // (x:ys) : group zs
            let result_chirho = cons_chirho(
                cons_chirho(
                    CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                ),
                rec_call_chirho,
            );

            // case pair of ($tuple2 ys zs) -> result
            let case_pair_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(pair_chirho.id_chirho)),
                bind_chirho: self.fresh_binder_chirho("_gp", a_chirho.clone()),
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![ys_chirho, zs_chirho],
                    rhs_chirho: result_chirho,
                }],
            };

            // let pair = span ... in case pair of ...
            let let_span_chirho = CoreExprChirho::LetChirho {
                rec_chirho: false,
                binds_chirho: vec![(pair_chirho, span_call_chirho)],
                body_chirho: Box::new(case_pair_chirho),
            };

            // case xs of [] -> []; (x:rest) -> let_span
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: nil_chirho(),
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![x_chirho, rest_chirho],
                        rhs_chirho: let_span_chirho,
                    },
                ],
            };

            let list_list_int_chirho =
                TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(int_chirho.clone()))));
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: group_id_chirho,
                    name_chirho: "group".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(int_chirho.clone())),
                        list_list_int_chirho,
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── iterate :: (a -> a) -> a -> [a] ──
        // Already exists in extra_list_prelude, skip if present
        // (this is just a placeholder comment to document coverage)

        // ── Data.Tuple: first, second, bimap ──
        // first :: (a -> b) -> (a, c) -> (b, c)
        // first f (a, c) = (f a, c)
        {
            let first_id_chirho = self.resolve_or_fresh_id_chirho("first");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
            );
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let pa_chirho = self.fresh_binder_chirho("pa", a_chirho.clone());
            let pc_chirho = self.fresh_binder_chirho("pc", c_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_fscr", a_chirho.clone());

            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(pa_chirho.id_chirho)),
            };
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    apply_f_chirho,
                    CoreExprChirho::VarChirho(pc_chirho.id_chirho),
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![pa_chirho, pc_chirho],
                    rhs_chirho: mk_tuple_chirho,
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: first_id_chirho,
                    name_chirho: "first".to_string(),
                    ty_chirho: a_chirho.clone(), // placeholder
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // second :: (b -> c) -> (a, b) -> (a, c)
        // second f (a, b) = (a, f b)
        {
            let second_id_chirho = self.resolve_or_fresh_id_chirho("second");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(b_chirho.clone(), c_chirho.clone()),
            );
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let pa_chirho = self.fresh_binder_chirho("pa", a_chirho.clone());
            let pb_chirho = self.fresh_binder_chirho("pb", b_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_sscr", a_chirho.clone());

            let apply_f_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(pb_chirho.id_chirho)),
            };
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![
                    CoreExprChirho::VarChirho(pa_chirho.id_chirho),
                    apply_f_chirho,
                ],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![pa_chirho, pb_chirho],
                    rhs_chirho: mk_tuple_chirho,
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: second_id_chirho,
                    name_chirho: "second".to_string(),
                    ty_chirho: a_chirho.clone(), // placeholder
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── both :: (a -> b) -> (a, a) -> (b, b) ──
        // both f (x, y) = (f x, f y)
        {
            let both_id_chirho = self.resolve_or_fresh_id_chirho("both");
            let f_chirho = self.fresh_binder_chirho(
                "f",
                TyChirho::fun_chirho(a_chirho.clone(), b_chirho.clone()),
            );
            let p_chirho = self.fresh_binder_chirho("p", a_chirho.clone());
            let px_chirho = self.fresh_binder_chirho("px", a_chirho.clone());
            let py_chirho = self.fresh_binder_chirho("py", a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_bscr", a_chirho.clone());

            let fx_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(px_chirho.id_chirho)),
            };
            let fy_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(f_chirho.id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(py_chirho.id_chirho)),
            };
            let mk_tuple_chirho = CoreExprChirho::ConAppChirho {
                con_name_chirho: "$tuple2".to_string(),
                args_chirho: vec![fx_chirho, fy_chirho],
            };
            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(p_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: a_chirho.clone(),
                alts_chirho: vec![CoreAltChirho {
                    con_chirho: AltConChirho::DataConChirho("$tuple2".to_string()),
                    binders_chirho: vec![px_chirho, py_chirho],
                    rhs_chirho: mk_tuple_chirho,
                }],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: f_chirho,
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: p_chirho,
                    body_chirho: Box::new(body_chirho),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: both_id_chirho,
                    name_chirho: "both".to_string(),
                    ty_chirho: a_chirho.clone(), // placeholder
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── tails :: [a] -> [[a]] ──
        // tails []     = [[]]
        // tails (x:xs) = (x:xs) : tails xs
        {
            let tails_id_chirho = self.resolve_or_fresh_id_chirho("tails");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_tscr", list_a_chirho.clone());

            let list_list_a_chirho = TyChirho::ListChirho(Box::new(list_a_chirho.clone()));

            // tails xs in the cons case
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(tails_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };
            // (h:t) : tails t
            let whole_list_chirho = cons_chirho(
                CoreExprChirho::VarChirho(h_chirho.id_chirho),
                CoreExprChirho::VarChirho(t_chirho.id_chirho),
            );
            let cons_case_chirho = cons_chirho(whole_list_chirho, rec_call_chirho);

            // [[]] — a list containing the empty list
            let singleton_nil_chirho = cons_chirho(nil_chirho(), nil_chirho());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: tails_id_chirho,
                    name_chirho: "tails".to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        list_a_chirho.clone(),
                        list_list_a_chirho.clone(),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // ── inits :: [a] -> [[a]] ──
        // inits []     = [[]]
        // inits (x:xs) = [] : map (x:) (inits xs)
        // We implement this using a helper that accumulates the prefix.
        // Simpler recursive version:
        // inits xs = [] : case xs of
        //   []   -> []
        //   x:xs' -> map (x:) (inits xs')
        // But map (x:) requires partial application. Instead we use a direct
        // recursive approach:
        // inits [] = [[]]
        // inits (x:xs) = [] : prependAll x (inits xs)
        // where prependAll x [[]]        = [[x]]
        //       prependAll x (ys:rest)    = (x:ys) : prependAll x rest
        //       prependAll x []           = []
        //
        // Actually the simplest correct definition:
        // inits []     = [[]]
        // inits (x:xs) = [] : map (\ys -> x : ys) (inits xs)
        // which needs map. Since map is already in the Prelude, we can reference it.
        {
            let inits_id_chirho = self.resolve_or_fresh_id_chirho("inits");
            let map_id_chirho = self.resolve_or_fresh_id_chirho("map");
            let xs_chirho = self.fresh_binder_chirho("xs", list_a_chirho.clone());
            let h_chirho = self.fresh_binder_chirho("h", a_chirho.clone());
            let t_chirho = self.fresh_binder_chirho("t", list_a_chirho.clone());
            let scr_chirho = self.fresh_binder_chirho("_iscr", list_a_chirho.clone());

            let list_list_a_chirho = TyChirho::ListChirho(Box::new(list_a_chirho.clone()));

            // (\ys -> h : ys)
            let ys_chirho = self.fresh_binder_chirho("ys", list_a_chirho.clone());
            let prepend_lam_chirho = CoreExprChirho::LamChirho {
                binder_chirho: ys_chirho.clone(),
                body_chirho: Box::new(cons_chirho(
                    CoreExprChirho::VarChirho(h_chirho.id_chirho),
                    CoreExprChirho::VarChirho(ys_chirho.id_chirho),
                )),
            };

            // inits t
            let rec_call_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::VarChirho(inits_id_chirho)),
                arg_chirho: Box::new(CoreExprChirho::VarChirho(t_chirho.id_chirho)),
            };

            // map (\ys -> h:ys) (inits t)
            let mapped_chirho = CoreExprChirho::AppChirho {
                fun_chirho: Box::new(CoreExprChirho::AppChirho {
                    fun_chirho: Box::new(CoreExprChirho::VarChirho(map_id_chirho)),
                    arg_chirho: Box::new(prepend_lam_chirho),
                }),
                arg_chirho: Box::new(rec_call_chirho),
            };

            // [] : map (\ys -> h:ys) (inits t)
            let cons_case_chirho = cons_chirho(nil_chirho(), mapped_chirho);

            // [[]]
            let singleton_nil_chirho = cons_chirho(nil_chirho(), nil_chirho());

            let body_chirho = CoreExprChirho::CaseChirho {
                scrutinee_chirho: Box::new(CoreExprChirho::VarChirho(xs_chirho.id_chirho)),
                bind_chirho: scr_chirho,
                result_ty_chirho: list_list_a_chirho.clone(),
                alts_chirho: vec![
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho("[]".to_string()),
                        binders_chirho: vec![],
                        rhs_chirho: singleton_nil_chirho,
                    },
                    CoreAltChirho {
                        con_chirho: AltConChirho::DataConChirho(":".to_string()),
                        binders_chirho: vec![h_chirho, t_chirho],
                        rhs_chirho: cons_case_chirho,
                    },
                ],
            };
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: xs_chirho,
                body_chirho: Box::new(body_chirho),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: inits_id_chirho,
                    name_chirho: "inits".to_string(),
                    ty_chirho: TyChirho::fun_chirho(list_a_chirho.clone(), list_list_a_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: true,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }

    /// Generate lazy recursive Core IR functions for arithmetic sequences.
    ///
    /// These replace the eager primops (which built entire lists synchronously)
    /// with recursive functions whose cons tails are thunked — giving true
    /// Haskell lazy evaluation for infinite lists.
    ///
    /// ```text
    /// enumFrom n       = n : enumFrom (n +# 1)
    /// enumFromThen a b = a : enumFromThen b (2*b - a)
    /// enumFromTo n to  = case n ># to of { True -> []; False -> n : enumFromTo (n+#1) to }
    /// enumFromThenTo a b to = let step = b -# a in go a step to
    ///   where go n s to | s >= 0    = if n > to then [] else n : go (n+s) s to
    ///                   | otherwise = if n < to then [] else n : go (n+s) s to
    /// ```
    fn generate_enum_sequence_prelude_chirho(&mut self) {
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
            let ef_id_chirho = self.resolve_or_fresh_id_chirho("enumFrom");
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
                    name_chirho: "enumFrom".to_string(),
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
            let eft_id_chirho = self.resolve_or_fresh_id_chirho("enumFromThen");
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
                    name_chirho: "enumFromThen".to_string(),
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
            let efto_id_chirho = self.resolve_or_fresh_id_chirho("enumFromTo");
            let n_chirho = self.fresh_binder_chirho("n", int_chirho.clone());
            let to_chirho = self.fresh_binder_chirho("to", int_chirho.clone());
            let w_chirho = self.fresh_binder_chirho("$w", TyChirho::bool_chirho());

            let n_gt_to_chirho = prim_bin_chirho(
                ">#",
                CoreExprChirho::VarChirho(n_chirho.id_chirho),
                CoreExprChirho::VarChirho(to_chirho.id_chirho),
            );

            let n_plus_1_chirho = prim_bin_chirho(
                "+#",
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
                    name_chirho: "enumFromTo".to_string(),
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
            let eftt_id_chirho = self.resolve_or_fresh_id_chirho("enumFromThenTo");
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
            // a +# 2*step = 2*b - a
            let next_b_chirho = prim_bin_chirho(
                "-#",
                prim_bin_chirho(
                    "*#",
                    CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(2)),
                    CoreExprChirho::VarChirho(b_chirho.id_chirho),
                ),
                CoreExprChirho::VarChirho(a_chirho.id_chirho),
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
                    name_chirho: "enumFromThenTo".to_string(),
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

    /// Generate `$prim_NFData_rnf_*` bindings and prelude-level `deepseq`, `force`, `evaluate`.
    ///
    /// For primitive types (Int, Char, Bool, Double), `rnf` is essentially `seq`:
    /// force to WHNF (which IS NF for primitives) and return `()`.
    /// `deepseq x y = rnf x `seq` y` — but since our runtime already forces to WHNF on
    /// seq, and primitives are already in NF at WHNF, we implement deepseq as seq.
    /// `force x = deepseq x x`
    /// `evaluate x = return x` (force to WHNF, which is what our STG evaluator does)
    fn generate_nfdata_prelude_chirho(&mut self) {
        let unit_ty_chirho = TyChirho::TupleChirho(vec![]);

        // $prim_NFData_rnf_Int, _Char, _Bool, _Double
        // All are: \x -> seq x ()
        for type_name_chirho in &["Int", "Char", "Bool", "Double"] {
            let prim_name_chirho = format!("$prim_NFData_rnf_{}", type_name_chirho);
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(&prim_name_chirho);
            let arg_ty_chirho = TyChirho::ConChirho(type_name_chirho.to_string());
            let x_chirho = self.fresh_binder_chirho("x", arg_ty_chirho.clone());

            // rnf x = x `seq` ()  ≈  seq# x ()
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "seq#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::LitChirho(CoreLitChirho::IntChirho(0)), // unit approximation
                    ],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(arg_ty_chirho, unit_ty_chirho.clone()),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level `deepseq :: a -> b -> b`
        // deepseq x y = x `seq` y
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(3410));
            let b_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(3411));
            let prim_name_chirho = "deepseq";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let y_chirho = self.fresh_binder_chirho("y", b_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: y_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                        name_chirho: "seq#".to_string(),
                        args_chirho: vec![
                            CoreExprChirho::VarChirho(x_chirho.id_chirho),
                            CoreExprChirho::VarChirho(y_chirho.id_chirho),
                        ],
                    }),
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(
                        a_chirho,
                        TyChirho::fun_chirho(b_chirho.clone(), b_chirho),
                    ),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level `force :: a -> a`
        // force x = x `seq` x
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(3412));
            let prim_name_chirho = "force";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::PrimOpChirho {
                    name_chirho: "seq#".to_string(),
                    args_chirho: vec![
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                        CoreExprChirho::VarChirho(x_chirho.id_chirho),
                    ],
                }),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }

        // Prelude-level `evaluate :: a -> IO a`
        // evaluate x = return x (force to WHNF, which is what STG evaluator does)
        {
            let a_chirho =
                TyChirho::VarChirho(haskelujah_typing_chirho::ty_chirho::TyVarChirho(3413));
            let prim_name_chirho = "evaluate";
            let prim_id_chirho = self.resolve_or_fresh_id_chirho(prim_name_chirho);
            let x_chirho = self.fresh_binder_chirho("x", a_chirho.clone());
            let rhs_chirho = CoreExprChirho::LamChirho {
                binder_chirho: x_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::VarChirho(x_chirho.id_chirho)),
            };
            self.generated_bindings_chirho.push(CoreBindingChirho {
                binder_chirho: BinderChirho {
                    id_chirho: prim_id_chirho,
                    name_chirho: prim_name_chirho.to_string(),
                    ty_chirho: TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                rhs_chirho,
                is_rec_chirho: false,
                inline_chirho: InlineAnnotationChirho::NoneChirho,
            });
        }
    }
}
