// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 13 of 14: original lines 19194-20139 of
//! `seed_builtins_chirho`, in order (37 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_13_chirho(env_chirho: &mut TyEnvChirho) {
    // Semigroup: (<>) :: Semigroup a => a -> a -> a
    {
        let a_chirho = TyVarChirho(7290);
        env_chirho.bind_chirho(
            "<>".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Semigroup".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // mappend :: Monoid a => a -> a -> a (same as <>)
    {
        let a_chirho = TyVarChirho(7300);
        env_chirho.bind_chirho(
            "mappend".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monoid".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // mempty :: Monoid a => a
    {
        let a_chirho = TyVarChirho(7310);
        env_chirho.bind_chirho(
            "mempty".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monoid".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::VarChirho(a_chirho),
            },
        );
    }

    // mconcat :: Monoid a => [a] -> a
    {
        let a_chirho = TyVarChirho(7320);
        env_chirho.bind_chirho(
            "mconcat".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monoid".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // getDual :: Dual a -> a, Dual :: a -> Dual a
    {
        let a_chirho = TyVarChirho(7330);
        env_chirho.bind_chirho(
            "Dual".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Dual".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
        env_chirho.bind_chirho(
            "getDual".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Dual".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // nub :: Eq a => [a] -> [a]
    {
        let a_chirho = TyVarChirho(7340);
        env_chirho.bind_chirho(
            "nub".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                ),
            },
        );
    }

    // group :: Eq a => [a] -> [[a]]
    {
        let a_chirho = TyVarChirho(7350);
        env_chirho.bind_chirho(
            "group".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                        TyChirho::VarChirho(a_chirho),
                    )))),
                ),
            },
        );
    }

    // forM :: Monad m => [a] -> (a -> m b) -> m [b]  (simplified as mapM with args swapped)
    {
        let m_chirho = TyVarChirho(7360);
        let a_chirho = TyVarChirho(7361);
        let b_chirho = TyVarChirho(7362);
        env_chirho.bind_chirho(
            "forM".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(m_chirho)),
                                Box::new(TyChirho::VarChirho(b_chirho)),
                            ),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                            b_chirho,
                        )))),
                    ),
                ),
            },
        );
    }

    // forM_ :: Monad m => [a] -> (a -> m b) -> m ()
    {
        let m_chirho = TyVarChirho(7370);
        let a_chirho = TyVarChirho(7371);
        let b_chirho = TyVarChirho(7372);
        env_chirho.bind_chirho(
            "forM_".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(m_chirho)),
                                Box::new(TyChirho::VarChirho(b_chirho)),
                            ),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // MonadZip methods
    {
        let m_chirho = TyVarChirho(7373);
        let a_chirho = TyVarChirho(7374);
        let b_chirho = TyVarChirho(7375);
        let c_chirho = TyVarChirho(7376);
        let ma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(m_chirho)),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let mb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(m_chirho)),
            Box::new(TyChirho::VarChirho(b_chirho)),
        );
        let mc_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(m_chirho)),
            Box::new(TyChirho::VarChirho(c_chirho)),
        );
        env_chirho.bind_chirho(
            "mzipWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "MonadZip".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(b_chirho),
                                TyChirho::VarChirho(c_chirho),
                            ),
                        ),
                        ma_chirho,
                        mb_chirho,
                    ],
                    mc_chirho,
                ),
            },
        );
    }

    // Contravariant methods
    {
        let f_chirho = TyVarChirho(7377);
        let a_chirho = TyVarChirho(7378);
        let b_chirho = TyVarChirho(7379);
        env_chirho.bind_chirho(
            "contramap".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Contravariant".to_string(),
                    ty_chirho: TyChirho::VarChirho(f_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(b_chirho),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::VarChirho(b_chirho)),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
    }

    // Data.Coerce: Coercible (type class — accept as constraint)
    // withDict :: (Class => r) -> Dict Class -> r (simplified)

    // Data.Type.Equality: (:~:) constructor — Refl :: a :~: a
    {
        let a_chirho = TyVarChirho(7380);
        env_chirho.bind_chirho(
            "Refl".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho(":~:".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // Data.Type.Equality: (:~~:) constructor — HRefl :: a :~~: a
    {
        let a_chirho = TyVarChirho(7381);
        env_chirho.bind_chirho(
            "HRefl".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho(":~~:".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // realToFrac :: (Real a, Fractional b) => a -> b
    {
        let a_chirho = TyVarChirho(7390);
        let b_chirho = TyVarChirho(7391);
        env_chirho.bind_chirho(
            "realToFrac".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![
                    SchemePredChirho {
                        class_name_chirho: "Real".to_string(),
                        ty_chirho: TyChirho::VarChirho(a_chirho),
                        extra_tys_chirho: vec![],
                    },
                    SchemePredChirho {
                        class_name_chirho: "Fractional".to_string(),
                        ty_chirho: TyChirho::VarChirho(b_chirho),
                        extra_tys_chirho: vec![],
                    },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::VarChirho(b_chirho)),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // fromIntegral :: (Integral a, Num b) => a -> b
    {
        let a_chirho = TyVarChirho(7392);
        let b_chirho = TyVarChirho(7393);
        env_chirho.bind_chirho(
            "fromIntegral".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![
                    SchemePredChirho {
                        class_name_chirho: "Integral".to_string(),
                        ty_chirho: TyChirho::VarChirho(a_chirho),
                        extra_tys_chirho: vec![],
                    },
                    SchemePredChirho {
                        class_name_chirho: "Num".to_string(),
                        ty_chirho: TyChirho::VarChirho(b_chirho),
                        extra_tys_chirho: vec![],
                    },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::VarChirho(b_chirho)),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // toInteger :: Integral a => a -> Integer
    {
        let a_chirho = TyVarChirho(7394);
        env_chirho.bind_chirho(
            "toInteger".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::ConChirho("Integer".to_string())),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // toRational :: Real a => a -> Rational
    {
        let a_chirho = TyVarChirho(7395);
        env_chirho.bind_chirho(
            "toRational".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Real".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::ConChirho("Rational".to_string())),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // toIntegralSized :: (Integral a, Integral b, Bits a, Bits b) => a -> Maybe b
    {
        let a_chirho = TyVarChirho(7396);
        let b_chirho = TyVarChirho(7397);
        env_chirho.bind_chirho(
            "toIntegralSized".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![
                    SchemePredChirho {
                        class_name_chirho: "Integral".to_string(),
                        ty_chirho: TyChirho::VarChirho(a_chirho),
                        extra_tys_chirho: vec![],
                    },
                    SchemePredChirho {
                        class_name_chirho: "Integral".to_string(),
                        ty_chirho: TyChirho::VarChirho(b_chirho),
                        extra_tys_chirho: vec![],
                    },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(b_chirho)),
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // floatRange :: RealFloat a => a -> (Int, Int)
    // floatDigits :: RealFloat a => a -> Int
    // floatRadix :: RealFloat a => a -> Integer
    // decodeFloat :: RealFloat a => a -> (Integer, Int)
    // isNaN :: RealFloat a => a -> Bool
    // isInfinite :: RealFloat a => a -> Bool
    {
        let rf_a_chirho = TyVarChirho(7398);
        let rf_pred_chirho = SchemePredChirho {
            class_name_chirho: "RealFloat".to_string(),
            ty_chirho: TyChirho::VarChirho(rf_a_chirho),
            extra_tys_chirho: vec![],
        };
        // floatRange :: RealFloat a => a -> (Int, Int)
        env_chirho.bind_chirho(
            "floatRange".to_string(),
            SchemeChirho {
                vars_chirho: vec![rf_a_chirho],
                preds_chirho: vec![rf_pred_chirho.clone()],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rf_a_chirho),
                    TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::int_chirho()]),
                ),
            },
        );
        // floatDigits :: RealFloat a => a -> Int
        env_chirho.bind_chirho(
            "floatDigits".to_string(),
            SchemeChirho {
                vars_chirho: vec![rf_a_chirho],
                preds_chirho: vec![rf_pred_chirho.clone()],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rf_a_chirho),
                    TyChirho::int_chirho(),
                ),
            },
        );
        // floatRadix :: RealFloat a => a -> Integer
        env_chirho.bind_chirho(
            "floatRadix".to_string(),
            SchemeChirho {
                vars_chirho: vec![rf_a_chirho],
                preds_chirho: vec![rf_pred_chirho.clone()],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rf_a_chirho),
                    TyChirho::ConChirho("Integer".to_string()),
                ),
            },
        );
        // decodeFloat :: RealFloat a => a -> (Integer, Int)
        env_chirho.bind_chirho(
            "decodeFloat".to_string(),
            SchemeChirho {
                vars_chirho: vec![rf_a_chirho],
                preds_chirho: vec![rf_pred_chirho.clone()],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rf_a_chirho),
                    TyChirho::TupleChirho(vec![
                        TyChirho::ConChirho("Integer".to_string()),
                        TyChirho::int_chirho(),
                    ]),
                ),
            },
        );
        // encodeFloat :: RealFloat a => Integer -> Int -> a
        env_chirho.bind_chirho(
            "encodeFloat".to_string(),
            SchemeChirho {
                vars_chirho: vec![rf_a_chirho],
                preds_chirho: vec![rf_pred_chirho.clone()],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ConChirho("Integer".to_string()),
                        TyChirho::int_chirho(),
                    ],
                    TyChirho::VarChirho(rf_a_chirho),
                ),
            },
        );
        // isNaN / isInfinite / isDenormalized / isNegativeZero / isIEEE :: RealFloat a => a -> Bool
        for name_chirho in &[
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                SchemeChirho {
                    vars_chirho: vec![rf_a_chirho],
                    preds_chirho: vec![rf_pred_chirho.clone()],
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::VarChirho(rf_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                },
            );
        }
    }

    // sequence/sequence_/mapM/mapM_ — already defined with list-specific types earlier in seed_builtins

    // error :: forall a. [Char] -> a (already exists, but ensure errorWithoutStackTrace too)
    {
        let a_chirho = TyVarChirho(7410);
        env_chirho.bind_chirho(
            "errorWithoutStackTrace".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::ListChirho(Box::new(TyChirho::char_chirho()))),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // Data.Coerce: coerce :: Coercible a b => a -> b
    // (already have coerce :: a -> b, add with Coercible constraint)

    // Integral class methods: div, mod, divMod, quot, rem, quotRem
    for (name_chirho, _) in &[("div", ()), ("mod", ()), ("quot", ()), ("rem", ())] {
        let a_chirho = TyVarChirho(7411);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(a_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // divMod, quotRem :: Integral a => a -> a -> (a, a)
    for name_chirho in &["divMod", "quotRem"] {
        let a_chirho = TyVarChirho(7412);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(a_chirho)),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(a_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(a_chirho),
                        ])),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // Data.IORef (already covered in runtime, ensure newIORef/readIORef/writeIORef/modifyIORef types)
    // IORef :: STRef RealWorld a -> IORef a
    {
        let a_chirho = TyVarChirho(7413);
        let stref_realworld_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(TyChirho::ConChirho("RealWorld".to_string())),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "IORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(stref_realworld_a_chirho, ioref_a_chirho),
            },
        );
    }

    // Data.STRef: STRef :: MutVar# s a -> STRef s a
    {
        let s_chirho = TyVarChirho(7414);
        let a_chirho = TyVarChirho(7415);
        let mutvar_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("MutVar#".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let stref_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "STRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(mutvar_sa_chirho, stref_sa_chirho),
            },
        );
    }

    // Data.STRef functions preserve the GHC-shaped surface. The unifier accepts
    // saturated ST uses where the runtime shim exposes the raw inner value.
    // newSTRef :: forall s a. a -> ST s (STRef s a)
    {
        let s_chirho = TyVarChirho(7416);
        let a_chirho = TyVarChirho(7417);
        let stref_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let st_sa_stref_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(stref_sa_chirho),
        );
        env_chirho.bind_chirho(
            "newSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    st_sa_stref_sa_chirho,
                ),
            },
        );
    }

    // readSTRef :: forall s a. STRef s a -> ST s a
    {
        let s_chirho = TyVarChirho(7418);
        let a_chirho = TyVarChirho(7419);
        let stref_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let st_sa_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "readSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(stref_sa_chirho, st_sa_a_chirho),
            },
        );
    }

    // writeSTRef :: forall s a. STRef s a -> a -> ST s ()
    {
        let s_chirho = TyVarChirho(7420);
        let a_chirho = TyVarChirho(7421);
        let stref_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let st_sa_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::unit_chirho()),
        );
        env_chirho.bind_chirho(
            "writeSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![stref_sa_chirho, TyChirho::VarChirho(a_chirho)],
                    st_sa_unit_chirho,
                ),
            },
        );
    }

    // modifySTRef :: forall s a. STRef s a -> (a -> a) -> ST s ()
    {
        let s_chirho = TyVarChirho(7422);
        let a_chirho = TyVarChirho(7423);
        let stref_sa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let st_sa_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(s_chirho)),
            )),
            Box::new(TyChirho::unit_chirho()),
        );
        env_chirho.bind_chirho(
            "modifySTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        stref_sa_chirho,
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(a_chirho),
                        ),
                    ],
                    st_sa_unit_chirho,
                ),
            },
        );
    }

    // ── Additional missing Prelude / Data.List type schemes ──

    // splitAt :: forall a. Int -> [a] -> ([a], [a])
    {
        let a_chirho = TyVarChirho(7500);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "splitAt".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), list_a_chirho.clone()],
                    TyChirho::TupleChirho(vec![list_a_chirho.clone(), list_a_chirho]),
                ),
            },
        );
    }

    // and :: [Bool] -> Bool
    env_chirho.bind_chirho(
        "and".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // or :: [Bool] -> Bool
    env_chirho.bind_chirho(
        "or".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::bool_chirho())),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // sortOn :: forall a b. Ord b => (a -> b) -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7501);
        let b_chirho = TyVarChirho(7502);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "sortOn".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(b_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(b_chirho),
                        ),
                        list_a_chirho.clone(),
                    ],
                    list_a_chirho,
                ),
            },
        );
    }

    // union :: forall a. Eq a => [a] -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7503);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "union".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho.clone(), list_a_chirho.clone()],
                    list_a_chirho,
                ),
            },
        );
    }

    // (\\) :: forall a. Eq a => [a] -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7507);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "\\\\".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho.clone(), list_a_chirho.clone()],
                    list_a_chirho,
                ),
            },
        );
    }
}
