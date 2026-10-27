// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 03 of 14: original lines 9718-10632 of
//! `seed_builtins_chirho`, in order (51 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_03_chirho(env_chirho: &mut TyEnvChirho) {
    let read_paren_a_chirho = TyVarChirho(1196);
    env_chirho.bind_chirho(
        "readParen".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_paren_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::bool_chirho(),
                    TyChirho::fun_chirho(
                        TyChirho::string_chirho(),
                        TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(read_paren_a_chirho),
                            TyChirho::string_chirho(),
                        ]))),
                    ),
                ],
                TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(read_paren_a_chirho),
                        TyChirho::string_chirho(),
                    ]))),
                ),
            ),
        },
    );
    env_chirho.bind_chirho(
        "lex".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                TyChirho::string_chirho(),
                TyChirho::string_chirho(),
            ]))),
        )),
    );

    // fromString :: forall a. IsString a => String -> a
    let fs_v_chirho = TyVarChirho(1197);
    env_chirho.bind_chirho(
        "fromString".to_string(),
        SchemeChirho {
            vars_chirho: vec![fs_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "IsString".to_string(),
                ty_chirho: TyChirho::VarChirho(fs_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::VarChirho(fs_v_chirho),
            ),
        },
    );

    // fromList :: forall a l. IsList l => [a] -> l
    let fl_v_chirho = TyVarChirho(1198);
    let fl_a_chirho = TyVarChirho(1199);
    env_chirho.bind_chirho(
        "fromList".to_string(),
        SchemeChirho {
            vars_chirho: vec![fl_v_chirho, fl_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "IsList".to_string(),
                ty_chirho: TyChirho::VarChirho(fl_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(fl_a_chirho))),
                TyChirho::VarChirho(fl_v_chirho),
            ),
        },
    );

    // toList :: forall a l. IsList l => l -> [a]
    env_chirho.bind_chirho(
        "toList".to_string(),
        SchemeChirho {
            vars_chirho: vec![fl_v_chirho, fl_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "IsList".to_string(),
                ty_chirho: TyChirho::VarChirho(fl_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(fl_v_chirho),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(fl_a_chirho))),
            ),
        },
    );

    // div :: Int -> Int -> Int  (Integral-specialized to Int)
    env_chirho.bind_chirho(
        "div".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // mod :: Int -> Int -> Int  (Integral-specialized to Int)
    env_chirho.bind_chirho(
        "mod".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // quot :: Int -> Int -> Int  (truncating division toward zero)
    env_chirho.bind_chirho(
        "quot".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // rem :: Int -> Int -> Int  (remainder of truncating division)
    env_chirho.bind_chirho(
        "rem".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // pure :: forall f a. Applicative f => a -> f a
    let pure_f_chirho = TyVarChirho(1500);
    let pure_a_chirho = TyVarChirho(1501);
    env_chirho.bind_chirho(
        "pure".to_string(),
        SchemeChirho {
            vars_chirho: vec![pure_f_chirho, pure_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Applicative".to_string(),
                ty_chirho: TyChirho::VarChirho(pure_f_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(pure_a_chirho),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(pure_f_chirho)),
                    Box::new(TyChirho::VarChirho(pure_a_chirho)),
                ),
            ),
        },
    );

    // (<*>) :: forall f a b. Applicative f => f (a -> b) -> f a -> f b
    let ap_f_chirho = TyVarChirho(1502);
    let ap_a_chirho = TyVarChirho(1503);
    let ap_b_chirho = TyVarChirho(1504);
    env_chirho.bind_chirho(
        "<*>".to_string(),
        SchemeChirho {
            vars_chirho: vec![ap_f_chirho, ap_a_chirho, ap_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Applicative".to_string(),
                ty_chirho: TyChirho::VarChirho(ap_f_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(ap_f_chirho)),
                        Box::new(TyChirho::fun_chirho(
                            TyChirho::VarChirho(ap_a_chirho),
                            TyChirho::VarChirho(ap_b_chirho),
                        )),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(ap_f_chirho)),
                        Box::new(TyChirho::VarChirho(ap_a_chirho)),
                    ),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(ap_f_chirho)),
                    Box::new(TyChirho::VarChirho(ap_b_chirho)),
                ),
            ),
        },
    );

    // id :: forall a. a -> a
    let id_var_chirho = TyVarChirho(1000);
    env_chirho.bind_chirho(
        "id".to_string(),
        SchemeChirho {
            vars_chirho: vec![id_var_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(id_var_chirho),
                TyChirho::VarChirho(id_var_chirho),
            ),
        },
    );

    // withFrozenCallStack :: forall a. a -> a
    // We treat this as an identity at type level so GHC.Stack re-exports
    // and Prelude-style availability typecheck cleanly.
    let with_frozen_call_stack_var_chirho = TyVarChirho(1000_1);
    env_chirho.bind_chirho(
        "withFrozenCallStack".to_string(),
        SchemeChirho {
            vars_chirho: vec![with_frozen_call_stack_var_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(with_frozen_call_stack_var_chirho),
                TyChirho::VarChirho(with_frozen_call_stack_var_chirho),
            ),
        },
    );

    // const :: forall a b. a -> b -> a
    let const_a_chirho = TyVarChirho(1001);
    let const_b_chirho = TyVarChirho(1002);
    env_chirho.bind_chirho(
        "const".to_string(),
        SchemeChirho {
            vars_chirho: vec![const_a_chirho, const_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(const_a_chirho),
                    TyChirho::VarChirho(const_b_chirho),
                ],
                TyChirho::VarChirho(const_a_chirho),
            ),
        },
    );

    // flip :: forall a b c. (a -> b -> c) -> b -> a -> c
    let flip_a_chirho = TyVarChirho(1003);
    let flip_b_chirho = TyVarChirho(1004);
    let flip_c_chirho = TyVarChirho(1005);
    env_chirho.bind_chirho(
        "flip".to_string(),
        SchemeChirho {
            vars_chirho: vec![flip_a_chirho, flip_b_chirho, flip_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(flip_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(flip_b_chirho),
                        TyChirho::VarChirho(flip_c_chirho),
                    ),
                ),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(flip_b_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(flip_a_chirho),
                        TyChirho::VarChirho(flip_c_chirho),
                    ),
                ),
            ),
        },
    );

    // even :: Integral a => a -> Bool
    {
        let v_chirho = TyVarChirho(1280);
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Integral".to_string(),
                ty_chirho: TyChirho::VarChirho(v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(v_chirho), TyChirho::bool_chirho()),
        };
        env_chirho.bind_chirho("even".to_string(), scheme_chirho.clone());
        env_chirho.bind_chirho("odd".to_string(), scheme_chirho);
    }

    // fst :: forall a b. (a, b) -> a
    let fst_a_chirho = TyVarChirho(1003);
    let fst_b_chirho = TyVarChirho(1004);
    env_chirho.bind_chirho(
        "fst".to_string(),
        SchemeChirho {
            vars_chirho: vec![fst_a_chirho, fst_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(fst_a_chirho),
                    TyChirho::VarChirho(fst_b_chirho),
                ]),
                TyChirho::VarChirho(fst_a_chirho),
            ),
        },
    );

    // snd :: forall a b. (a, b) -> b
    let snd_a_chirho = TyVarChirho(1005);
    let snd_b_chirho = TyVarChirho(1006);
    env_chirho.bind_chirho(
        "snd".to_string(),
        SchemeChirho {
            vars_chirho: vec![snd_a_chirho, snd_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(snd_a_chirho),
                    TyChirho::VarChirho(snd_b_chirho),
                ]),
                TyChirho::VarChirho(snd_b_chirho),
            ),
        },
    );

    // curry :: forall a b c. ((a, b) -> c) -> a -> b -> c
    let curry_a_chirho = TyVarChirho(1007);
    let curry_b_chirho = TyVarChirho(1008);
    let curry_c_chirho = TyVarChirho(1009);
    env_chirho.bind_chirho(
        "curry".to_string(),
        SchemeChirho {
            vars_chirho: vec![curry_a_chirho, curry_b_chirho, curry_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(curry_a_chirho),
                            TyChirho::VarChirho(curry_b_chirho),
                        ]),
                        TyChirho::VarChirho(curry_c_chirho),
                    ),
                    TyChirho::VarChirho(curry_a_chirho),
                    TyChirho::VarChirho(curry_b_chirho),
                ],
                TyChirho::VarChirho(curry_c_chirho),
            ),
        },
    );

    // uncurry :: forall a b c. (a -> b -> c) -> (a, b) -> c
    let uncurry_a_chirho = TyVarChirho(1010);
    let uncurry_b_chirho = TyVarChirho(1011);
    let uncurry_c_chirho = TyVarChirho(1012);
    env_chirho.bind_chirho(
        "uncurry".to_string(),
        SchemeChirho {
            vars_chirho: vec![uncurry_a_chirho, uncurry_b_chirho, uncurry_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(uncurry_a_chirho),
                        TyChirho::VarChirho(uncurry_b_chirho),
                    ],
                    TyChirho::VarChirho(uncurry_c_chirho),
                ),
                TyChirho::fun_chirho(
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(uncurry_a_chirho),
                        TyChirho::VarChirho(uncurry_b_chirho),
                    ]),
                    TyChirho::VarChirho(uncurry_c_chirho),
                ),
            ),
        },
    );

    // take :: forall a. Int -> [a] -> [a]
    {
        let take_a_chirho = TyVarChirho(3200);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(take_a_chirho)));
        env_chirho.bind_chirho(
            "take".to_string(),
            SchemeChirho {
                vars_chirho: vec![take_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho),
                ),
            },
        );
    }

    // drop :: forall a. Int -> [a] -> [a]
    {
        let drop_a_chirho = TyVarChirho(3210);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(drop_a_chirho)));
        env_chirho.bind_chirho(
            "drop".to_string(),
            SchemeChirho {
                vars_chirho: vec![drop_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::fun_chirho(list_a_chirho.clone(), list_a_chirho),
                ),
            },
        );
    }

    // words :: String -> [String]
    env_chirho.bind_chirho(
        "words".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
        )),
    );

    // unwords :: [String] -> String
    env_chirho.bind_chirho(
        "unwords".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
            TyChirho::string_chirho(),
        )),
    );

    // concat :: forall a. [[a]] -> [a]
    let concat_elem_var_chirho = TyVarChirho(3064);
    env_chirho.bind_chirho(
        "concat".to_string(),
        SchemeChirho {
            vars_chirho: vec![concat_elem_var_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                    TyChirho::VarChirho(concat_elem_var_chirho),
                )))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(concat_elem_var_chirho))),
            ),
        },
    );

    // intercalate :: String -> [String] -> String
    env_chirho.bind_chirho(
        "intercalate".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
                TyChirho::string_chirho(),
            ),
        )),
    );

    // toInteger :: Int -> Integer
    env_chirho.bind_chirho(
        "toInteger".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Integer".to_string()),
        )),
    );

    // ceiling/floor/round/truncate :: (RealFrac a, Integral b) => a -> b
    for name_chirho in ["ceiling", "floor", "round", "truncate"] {
        let realfrac_a_chirho = TyVarChirho(10101);
        let integral_b_chirho = TyVarChirho(10102);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![realfrac_a_chirho, integral_b_chirho],
                preds_chirho: vec![
                    SchemePredChirho {
                        class_name_chirho: "RealFrac".to_string(),
                        ty_chirho: TyChirho::VarChirho(realfrac_a_chirho),
                        extra_tys_chirho: vec![],
                    },
                    SchemePredChirho {
                        class_name_chirho: "Integral".to_string(),
                        ty_chirho: TyChirho::VarChirho(integral_b_chirho),
                        extra_tys_chirho: vec![],
                    },
                ],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(realfrac_a_chirho),
                    TyChirho::VarChirho(integral_b_chirho),
                ),
            },
        );
    }

    // properFraction :: (RealFrac a, Integral b) => a -> (b, a)
    let proper_fraction_a_chirho = TyVarChirho(10103);
    let proper_fraction_b_chirho = TyVarChirho(10104);
    env_chirho.bind_chirho(
        "properFraction".to_string(),
        SchemeChirho {
            vars_chirho: vec![proper_fraction_a_chirho, proper_fraction_b_chirho],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "RealFrac".to_string(),
                    ty_chirho: TyChirho::VarChirho(proper_fraction_a_chirho),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(proper_fraction_b_chirho),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(proper_fraction_a_chirho),
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(proper_fraction_b_chirho),
                    TyChirho::VarChirho(proper_fraction_a_chirho),
                ]),
            ),
        },
    );

    // isJust :: forall f a. f a -> Bool
    // These generated helpers inspect the `Nothing`/`Just` constructor shape,
    // so allow local Maybe-like declarations without conflating tycons globally.
    let is_just_a_chirho = TyVarChirho(1020);
    let is_just_f_chirho = TyVarChirho(10200);
    env_chirho.bind_chirho(
        "isJust".to_string(),
        SchemeChirho {
            vars_chirho: vec![is_just_f_chirho, is_just_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(is_just_f_chirho)),
                    Box::new(TyChirho::VarChirho(is_just_a_chirho)),
                ),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // isNothing :: forall f a. f a -> Bool
    let is_nothing_a_chirho = TyVarChirho(1021);
    let is_nothing_f_chirho = TyVarChirho(10201);
    env_chirho.bind_chirho(
        "isNothing".to_string(),
        SchemeChirho {
            vars_chirho: vec![is_nothing_f_chirho, is_nothing_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(is_nothing_f_chirho)),
                    Box::new(TyChirho::VarChirho(is_nothing_a_chirho)),
                ),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // fromMaybe :: forall f a. a -> f a -> a
    let from_maybe_a_chirho = TyVarChirho(1022);
    let from_maybe_f_chirho = TyVarChirho(10202);
    env_chirho.bind_chirho(
        "fromMaybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![from_maybe_f_chirho, from_maybe_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(from_maybe_a_chirho),
                TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(from_maybe_f_chirho)),
                        Box::new(TyChirho::VarChirho(from_maybe_a_chirho)),
                    ),
                    TyChirho::VarChirho(from_maybe_a_chirho),
                ),
            ),
        },
    );

    // maybe :: forall f a b. b -> (a -> b) -> f a -> b
    let maybe_a_chirho = TyVarChirho(1023);
    let maybe_b_chirho = TyVarChirho(1024);
    let maybe_f_chirho = TyVarChirho(10203);
    env_chirho.bind_chirho(
        "maybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![maybe_f_chirho, maybe_a_chirho, maybe_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(maybe_b_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(maybe_a_chirho),
                        TyChirho::VarChirho(maybe_b_chirho),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(maybe_f_chirho)),
                        Box::new(TyChirho::VarChirho(maybe_a_chirho)),
                    ),
                ],
                TyChirho::VarChirho(maybe_b_chirho),
            ),
        },
    );

    // maybeToList :: forall a. Maybe a -> [a]
    let mtl_a_chirho = TyVarChirho(1030);
    env_chirho.bind_chirho(
        "maybeToList".to_string(),
        SchemeChirho {
            vars_chirho: vec![mtl_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(mtl_a_chirho)),
                ),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mtl_a_chirho))),
            ),
        },
    );

    // listToMaybe :: forall a. [a] -> Maybe a
    let ltm_a_chirho = TyVarChirho(1031);
    env_chirho.bind_chirho(
        "listToMaybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![ltm_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ltm_a_chirho))),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(ltm_a_chirho)),
                ),
            ),
        },
    );

    // catMaybes :: forall a. [Maybe a] -> [a]
    let cm_a_chirho = TyVarChirho(1032);
    env_chirho.bind_chirho(
        "catMaybes".to_string(),
        SchemeChirho {
            vars_chirho: vec![cm_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(cm_a_chirho)),
                ))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_a_chirho))),
            ),
        },
    );

    // mapMaybe :: forall a b. (a -> Maybe b) -> [a] -> [b]
    let mm_a_chirho = TyVarChirho(1033);
    let mm_b_chirho = TyVarChirho(1034);
    env_chirho.bind_chirho(
        "mapMaybe".to_string(),
        SchemeChirho {
            vars_chirho: vec![mm_a_chirho, mm_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(mm_a_chirho),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Maybe".to_string())),
                            Box::new(TyChirho::VarChirho(mm_b_chirho)),
                        ),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mm_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mm_b_chirho))),
            ),
        },
    );

    // fromJust :: forall a. Maybe a -> a
    let fj_a_chirho = TyVarChirho(1035);
    env_chirho.bind_chirho(
        "fromJust".to_string(),
        SchemeChirho {
            vars_chirho: vec![fj_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(fj_a_chirho)),
                ),
                TyChirho::VarChirho(fj_a_chirho),
            ),
        },
    );

    // swap :: forall p a b. p a b -> p b a
    // This covers both tuple swap and Data.Bifunctor.Swap.swap.
    let sw_p_chirho = TyVarChirho(1035);
    let sw_a_chirho = TyVarChirho(1036);
    let sw_b_chirho = TyVarChirho(1037);
    let swap_arg_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(sw_p_chirho)),
            Box::new(TyChirho::VarChirho(sw_a_chirho)),
        )),
        Box::new(TyChirho::VarChirho(sw_b_chirho)),
    );
    let swap_result_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(sw_p_chirho)),
            Box::new(TyChirho::VarChirho(sw_b_chirho)),
        )),
        Box::new(TyChirho::VarChirho(sw_a_chirho)),
    );
    let swap_scheme_chirho = SchemeChirho {
        vars_chirho: vec![sw_p_chirho, sw_a_chirho, sw_b_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(swap_arg_ty_chirho, swap_result_ty_chirho),
    };
    env_chirho.bind_chirho("swap".to_string(), swap_scheme_chirho.clone());
    env_chirho.bind_chirho("Data.Bifunctor.Swap.swap".to_string(), swap_scheme_chirho);

    // Left :: forall a b. a -> Either a b
    let left_a_chirho = TyVarChirho(1038);
    let left_b_chirho = TyVarChirho(1039);
    env_chirho.bind_chirho(
        "Left".to_string(),
        SchemeChirho {
            vars_chirho: vec![left_a_chirho, left_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(left_a_chirho),
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Either".to_string())),
                        Box::new(TyChirho::VarChirho(left_a_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(left_b_chirho)),
                ),
            ),
        },
    );

    // Right :: forall a b. b -> Either a b
    let right_a_chirho = TyVarChirho(1040);
    let right_b_chirho = TyVarChirho(1041);
    env_chirho.bind_chirho(
        "Right".to_string(),
        SchemeChirho {
            vars_chirho: vec![right_a_chirho, right_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(right_b_chirho),
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Either".to_string())),
                        Box::new(TyChirho::VarChirho(right_a_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(right_b_chirho)),
                ),
            ),
        },
    );

    // either :: forall a b c. (a -> c) -> (b -> c) -> Either a b -> c
    let either_a_chirho = TyVarChirho(1025);
    let either_b_chirho = TyVarChirho(1026);
    let either_c_chirho = TyVarChirho(1027);
    env_chirho.bind_chirho(
        "either".to_string(),
        SchemeChirho {
            vars_chirho: vec![either_a_chirho, either_b_chirho, either_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(either_a_chirho),
                        TyChirho::VarChirho(either_c_chirho),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(either_b_chirho),
                        TyChirho::VarChirho(either_c_chirho),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Either".to_string())),
                            Box::new(TyChirho::VarChirho(either_a_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(either_b_chirho)),
                    ),
                ],
                TyChirho::VarChirho(either_c_chirho),
            ),
        },
    );

    // otherwise :: Bool  (otherwise = True)
    env_chirho.bind_chirho(
        "otherwise".to_string(),
        SchemeChirho::mono_chirho(TyChirho::bool_chirho()),
    );

    // ($) :: forall a b. (a -> b) -> a -> b
    let dollar_a_chirho = TyVarChirho(2000);
    let dollar_b_chirho = TyVarChirho(2001);
    env_chirho.bind_chirho(
        "$".to_string(),
        SchemeChirho {
            vars_chirho: vec![dollar_a_chirho, dollar_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dollar_a_chirho),
                        TyChirho::VarChirho(dollar_b_chirho),
                    ),
                    TyChirho::VarChirho(dollar_a_chirho),
                ],
                TyChirho::VarChirho(dollar_b_chirho),
            ),
        },
    );

    // ($!) :: forall a b. (a -> b) -> a -> b  (strict application)
    let bang_dollar_a_chirho = TyVarChirho(2010);
    let bang_dollar_b_chirho = TyVarChirho(2011);
    env_chirho.bind_chirho(
        "$!".to_string(),
        SchemeChirho {
            vars_chirho: vec![bang_dollar_a_chirho, bang_dollar_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(bang_dollar_a_chirho),
                        TyChirho::VarChirho(bang_dollar_b_chirho),
                    ),
                    TyChirho::VarChirho(bang_dollar_a_chirho),
                ],
                TyChirho::VarChirho(bang_dollar_b_chirho),
            ),
        },
    );

    // ($!!) :: forall a b. NFData a => (a -> b) -> a -> b
    // Deep strict application: f $!! x = deepseq x (f x)
    let dblbang_a_chirho = TyVarChirho(2090);
    let dblbang_b_chirho = TyVarChirho(2091);
    env_chirho.bind_chirho(
        "$!!".to_string(),
        SchemeChirho {
            vars_chirho: vec![dblbang_a_chirho, dblbang_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dblbang_a_chirho),
                        TyChirho::VarChirho(dblbang_b_chirho),
                    ),
                    TyChirho::VarChirho(dblbang_a_chirho),
                ],
                TyChirho::VarChirho(dblbang_b_chirho),
            ),
        },
    );

    // (.) :: forall a b c. (b -> c) -> (a -> b) -> a -> c
    let dot_a_chirho = TyVarChirho(2100);
    let dot_b_chirho = TyVarChirho(2101);
    let dot_c_chirho = TyVarChirho(2102);
    env_chirho.bind_chirho(
        ".".to_string(),
        SchemeChirho {
            vars_chirho: vec![dot_a_chirho, dot_b_chirho, dot_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dot_b_chirho),
                        TyChirho::VarChirho(dot_c_chirho),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(dot_a_chirho),
                        TyChirho::VarChirho(dot_b_chirho),
                    ),
                ],
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(dot_a_chirho),
                    TyChirho::VarChirho(dot_c_chirho),
                ),
            ),
        },
    );

    // negate :: forall a. Num a => a -> a
    let neg_v_chirho = TyVarChirho(2200);
    env_chirho.bind_chirho(
        "negate".to_string(),
        SchemeChirho {
            vars_chirho: vec![neg_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(neg_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(neg_v_chirho),
                TyChirho::VarChirho(neg_v_chirho),
            ),
        },
    );

    // -- I/O functions (IO a is a proper type constructor) --

    // putStrLn :: String -> IO ()
    env_chirho.bind_chirho(
        "putStrLn".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // putStr :: String -> IO ()
    env_chirho.bind_chirho(
        "putStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // print :: forall a. Show a => a -> IO ()
    let print_v_chirho = TyVarChirho(1600);
    env_chirho.bind_chirho(
        "print".to_string(),
        SchemeChirho {
            vars_chirho: vec![print_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(print_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(print_v_chirho),
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        },
    );
}
