// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Ordinary first-class Prelude combinators with real Core function bodies.
//! Workflow: language-features-chirho/dictionary-evidence-chirho.
use super::{
    BinderChirho, CoreBindingChirho, CoreExprChirho, DictPassCtxChirho, InlineAnnotationChirho,
    SpanChirho, TyChirho, TyVarChirho,
};

type PreludeBodyChirho = Box<dyn FnOnce(&mut DictPassCtxChirho) -> CoreExprChirho>;

impl DictPassCtxChirho {
    pub(super) fn generate_function_prelude_chirho(&mut self) {
        let bool_ty_chirho = TyChirho::bool_chirho();

        // Helper to generate a Prelude binding with ID matching.
        // Uses resolve_or_fresh_id_chirho so the binding ID matches
        // any reference the desugarer may have already created.
        let prelude_fns_chirho: Vec<(&str, TyChirho, PreludeBodyChirho)> = vec![
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
            // asTypeOf :: a -> a -> a
            (
                "asTypeOf",
                {
                    let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
                    TyChirho::fun_chirho(
                        a_chirho.clone(),
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    )
                },
                Box::new(|ctx_chirho: &mut Self| {
                    let a_chirho = TyChirho::VarChirho(TyVarChirho(9990));
                    let x_chirho = ctx_chirho.fresh_binder_chirho("x", a_chirho.clone());
                    let y_chirho = ctx_chirho.fresh_binder_chirho("y", a_chirho);
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
        self.generate_composition_chirho();
    }

    fn generate_composition_chirho(&mut self) {
        // A user definition has authority over the fallback Prelude body.
        if self.body_backed_names_chirho.contains_key(".") {
            return;
        }
        let input_ty_chirho = TyChirho::VarChirho(TyVarChirho(9990));
        let middle_ty_chirho = TyChirho::VarChirho(TyVarChirho(9991));
        let output_ty_chirho = TyChirho::VarChirho(TyVarChirho(9992));
        let first_ty_chirho =
            TyChirho::fun_chirho(middle_ty_chirho.clone(), output_ty_chirho.clone());
        let second_ty_chirho = TyChirho::fun_chirho(input_ty_chirho.clone(), middle_ty_chirho);
        let first_chirho =
            self.fresh_binder_chirho("compose_first_chirho", first_ty_chirho.clone());
        let second_chirho =
            self.fresh_binder_chirho("compose_second_chirho", second_ty_chirho.clone());
        let value_chirho =
            self.fresh_binder_chirho("compose_value_chirho", input_ty_chirho.clone());
        let id_chirho = self.resolve_or_fresh_id_chirho(".");
        self.generated_bindings_chirho.push(CoreBindingChirho {
            binder_chirho: BinderChirho {
                id_chirho,
                name_chirho: ".".to_owned(),
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![first_ty_chirho, second_ty_chirho, input_ty_chirho],
                    output_ty_chirho,
                ),
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
            rhs_chirho: CoreExprChirho::LamChirho {
                binder_chirho: first_chirho.clone(),
                body_chirho: Box::new(CoreExprChirho::LamChirho {
                    binder_chirho: second_chirho.clone(),
                    body_chirho: Box::new(CoreExprChirho::LamChirho {
                        binder_chirho: value_chirho.clone(),
                        body_chirho: Box::new(CoreExprChirho::AppChirho {
                            fun_chirho: Box::new(CoreExprChirho::VarChirho(first_chirho.id_chirho)),
                            arg_chirho: Box::new(CoreExprChirho::AppChirho {
                                fun_chirho: Box::new(CoreExprChirho::VarChirho(
                                    second_chirho.id_chirho,
                                )),
                                arg_chirho: Box::new(CoreExprChirho::VarChirho(
                                    value_chirho.id_chirho,
                                )),
                            }),
                        }),
                    }),
                }),
            },
            is_rec_chirho: false,
            inline_chirho: InlineAnnotationChirho::NoneChirho,
        });
    }
}
