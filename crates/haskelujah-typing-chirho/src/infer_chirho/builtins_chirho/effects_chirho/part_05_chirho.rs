// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 05 of 14: original lines 11594-12542 less the four hoisted closures of
//! `seed_builtins_chirho`, in order (51 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_05_chirho(env_chirho: &mut TyEnvChirho) {
    {
        let array_elem_chirho = TyVarChirho(1699);
        let uarray_int_elem_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("UArray".to_string())),
                Box::new(TyChirho::int_chirho()),
            )),
            Box::new(TyChirho::VarChirho(array_elem_chirho)),
        );
        let unsafe_at_scheme_chirho = SchemeChirho {
            vars_chirho: vec![array_elem_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![uarray_int_elem_ty_chirho, TyChirho::int_chirho()],
                TyChirho::VarChirho(array_elem_chirho),
            ),
        };
        env_chirho.bind_chirho("unsafeAt".to_string(), unsafe_at_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.Array.Base.unsafeAt".to_string(),
            unsafe_at_scheme_chirho,
        );
    }

    // unsafeIOToST :: IO a -> ST s a
    {
        let a_chirho = TyVarChirho(1693);
        let s_chirho = TyVarChirho(1694);
        env_chirho.bind_chirho(
            "unsafeIOToST".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, s_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::io_chirho(TyChirho::VarChirho(a_chirho))),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("ST".to_string())),
                            Box::new(TyChirho::VarChirho(s_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    let smgen_ty_chirho = TyChirho::ConChirho("SMGen".to_string());
    let word32_ty_chirho = TyChirho::ConChirho("Word32".to_string());
    let word64_ty_chirho = TyChirho::ConChirho("Word64".to_string());
    env_chirho.bind_chirho(
        "newSMGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(smgen_ty_chirho.clone())),
    );
    env_chirho.bind_chirho(
        "initSMGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(smgen_ty_chirho.clone())),
    );
    let mk_smgen_a_chirho = TyVarChirho(1691);
    env_chirho.bind_chirho(
        "mkSMGen".to_string(),
        SchemeChirho {
            vars_chirho: vec![mk_smgen_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Integral".to_string(),
                ty_chirho: TyChirho::VarChirho(mk_smgen_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(mk_smgen_a_chirho),
                smgen_ty_chirho.clone(),
            ),
        },
    );
    env_chirho.bind_chirho(
        "splitSMGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![smgen_ty_chirho.clone(), smgen_ty_chirho.clone()]),
        )),
    );
    env_chirho.bind_chirho(
        "nextInt".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![TyChirho::int_chirho(), smgen_ty_chirho.clone()]),
        )),
    );
    env_chirho.bind_chirho(
        "nextWord64".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![word64_ty_chirho.clone(), smgen_ty_chirho.clone()]),
        )),
    );
    env_chirho.bind_chirho(
        "nextWord32".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![word32_ty_chirho, smgen_ty_chirho.clone()]),
        )),
    );
    env_chirho.bind_chirho(
        "seedSMGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![word64_ty_chirho.clone(), word64_ty_chirho.clone()],
            smgen_ty_chirho.clone(),
        )),
    );
    env_chirho.bind_chirho(
        "unseedSMGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![word64_ty_chirho.clone(), word64_ty_chirho.clone()]),
        )),
    );
    env_chirho.bind_chirho(
        "nextDouble".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![TyChirho::double_chirho(), smgen_ty_chirho.clone()]),
        )),
    );
    env_chirho.bind_chirho(
        "nextFloat".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            smgen_ty_chirho.clone(),
            TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("Float".to_string()),
                smgen_ty_chirho.clone(),
            ]),
        )),
    );
    env_chirho.bind_chirho(
        "nextInteger".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Integer".to_string()),
                TyChirho::ConChirho("Integer".to_string()),
                smgen_ty_chirho.clone(),
            ],
            TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("Integer".to_string()),
                smgen_ty_chirho.clone(),
            ]),
        )),
    );
    env_chirho.bind_chirho(
        "bitmaskWithRejection64'".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![word64_ty_chirho.clone(), smgen_ty_chirho.clone()],
            TyChirho::TupleChirho(vec![word64_ty_chirho, smgen_ty_chirho]),
        )),
    );

    let liftm4_m_chirho = TyVarChirho(1698);
    let liftm4_a_chirho = TyVarChirho(1699);
    let liftm4_b_chirho = TyVarChirho(1702);
    let liftm4_c_chirho = TyVarChirho(1703);
    let liftm4_d_chirho = TyVarChirho(1704);
    let liftm4_r_chirho = TyVarChirho(1705);
    let liftm4_scheme_chirho = SchemeChirho {
        vars_chirho: vec![
            liftm4_m_chirho,
            liftm4_a_chirho,
            liftm4_b_chirho,
            liftm4_c_chirho,
            liftm4_d_chirho,
            liftm4_r_chirho,
        ],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Monad".to_string(),
            ty_chirho: TyChirho::VarChirho(liftm4_m_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(liftm4_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(liftm4_b_chirho),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(liftm4_c_chirho),
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(liftm4_d_chirho),
                                TyChirho::VarChirho(liftm4_r_chirho),
                            ),
                        ),
                    ),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm4_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm4_a_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm4_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm4_b_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm4_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm4_c_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm4_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm4_d_chirho)),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(liftm4_m_chirho)),
                Box::new(TyChirho::VarChirho(liftm4_r_chirho)),
            ),
        ),
    };
    env_chirho.bind_chirho("liftM4".to_string(), liftm4_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.liftM4".to_string(), liftm4_scheme_chirho);

    let liftm5_m_chirho = TyVarChirho(1706);
    let liftm5_a_chirho = TyVarChirho(1707);
    let liftm5_b_chirho = TyVarChirho(1708);
    let liftm5_c_chirho = TyVarChirho(1709);
    let liftm5_d_chirho = TyVarChirho(1710);
    let liftm5_e_chirho = TyVarChirho(1711);
    let liftm5_r_chirho = TyVarChirho(1712);
    let liftm5_scheme_chirho = SchemeChirho {
        vars_chirho: vec![
            liftm5_m_chirho,
            liftm5_a_chirho,
            liftm5_b_chirho,
            liftm5_c_chirho,
            liftm5_d_chirho,
            liftm5_e_chirho,
            liftm5_r_chirho,
        ],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Monad".to_string(),
            ty_chirho: TyChirho::VarChirho(liftm5_m_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(liftm5_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(liftm5_b_chirho),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(liftm5_c_chirho),
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(liftm5_d_chirho),
                                TyChirho::fun_chirho(
                                    TyChirho::VarChirho(liftm5_e_chirho),
                                    TyChirho::VarChirho(liftm5_r_chirho),
                                ),
                            ),
                        ),
                    ),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm5_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm5_a_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm5_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm5_b_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm5_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm5_c_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm5_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm5_d_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(liftm5_m_chirho)),
                    Box::new(TyChirho::VarChirho(liftm5_e_chirho)),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(liftm5_m_chirho)),
                Box::new(TyChirho::VarChirho(liftm5_r_chirho)),
            ),
        ),
    };
    env_chirho.bind_chirho("liftM5".to_string(), liftm5_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.liftM5".to_string(), liftm5_scheme_chirho);

    let zipwithm_m_chirho = TyVarChirho(1713);
    let zipwithm_a_chirho = TyVarChirho(1714);
    let zipwithm_b_chirho = TyVarChirho(1715);
    let zipwithm_c_chirho = TyVarChirho(1716);
    let zipwithm_scheme_chirho = SchemeChirho {
        vars_chirho: vec![
            zipwithm_m_chirho,
            zipwithm_a_chirho,
            zipwithm_b_chirho,
            zipwithm_c_chirho,
        ],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Monad".to_string(),
            ty_chirho: TyChirho::VarChirho(zipwithm_m_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(zipwithm_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(zipwithm_b_chirho),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(zipwithm_m_chirho)),
                            Box::new(TyChirho::VarChirho(zipwithm_c_chirho)),
                        ),
                    ),
                ),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipwithm_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipwithm_b_chirho))),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(zipwithm_m_chirho)),
                Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                    zipwithm_c_chirho,
                )))),
            ),
        ),
    };
    env_chirho.bind_chirho("zipWithM".to_string(), zipwithm_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.zipWithM".to_string(), zipwithm_scheme_chirho);

    let zipwithm_unit_scheme_chirho = SchemeChirho {
        vars_chirho: vec![
            zipwithm_m_chirho,
            zipwithm_a_chirho,
            zipwithm_b_chirho,
            zipwithm_c_chirho,
        ],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Monad".to_string(),
            ty_chirho: TyChirho::VarChirho(zipwithm_m_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(zipwithm_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(zipwithm_b_chirho),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(zipwithm_m_chirho)),
                            Box::new(TyChirho::VarChirho(zipwithm_c_chirho)),
                        ),
                    ),
                ),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipwithm_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipwithm_b_chirho))),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(zipwithm_m_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    env_chirho.bind_chirho("zipWithM_".to_string(), zipwithm_unit_scheme_chirho.clone());
    env_chirho.bind_chirho(
        "Control.Monad.zipWithM_".to_string(),
        zipwithm_unit_scheme_chirho,
    );

    // return :: forall m a. Monad m => a -> m a
    let return_m_chirho = TyVarChirho(1700);
    let return_a_chirho = TyVarChirho(1701);
    env_chirho.bind_chirho(
        "return".to_string(),
        SchemeChirho {
            vars_chirho: vec![return_m_chirho, return_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monad".to_string(),
                ty_chirho: TyChirho::VarChirho(return_m_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(return_a_chirho),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(return_m_chirho)),
                    Box::new(TyChirho::VarChirho(return_a_chirho)),
                ),
            ),
        },
    );
    env_chirho.bind_chirho(
        "Control.Monad.return".to_string(),
        env_chirho
            .lookup_chirho("return")
            .cloned()
            .expect("return should be seeded before its qualified alias"),
    );

    // fail :: forall m a. MonadFail m => String -> m a
    let fail_m_chirho = TyVarChirho(1702);
    let fail_a_chirho = TyVarChirho(1703);
    let fail_scheme_chirho = SchemeChirho {
        vars_chirho: vec![fail_m_chirho, fail_a_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "MonadFail".to_string(),
            ty_chirho: TyChirho::VarChirho(fail_m_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(fail_m_chirho)),
                Box::new(TyChirho::VarChirho(fail_a_chirho)),
            ),
        ),
    };
    env_chirho.bind_chirho("fail".to_string(), fail_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.Fail.fail".to_string(), fail_scheme_chirho);

    // mfix :: forall m a. MonadFix m => (a -> m a) -> m a
    let mfix_m_chirho = TyVarChirho(1720);
    let mfix_a_chirho = TyVarChirho(1721);
    let mfix_result_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::VarChirho(mfix_m_chirho)),
        Box::new(TyChirho::VarChirho(mfix_a_chirho)),
    );
    let mfix_scheme_chirho = SchemeChirho {
        vars_chirho: vec![mfix_m_chirho, mfix_a_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "MonadFix".to_string(),
            ty_chirho: TyChirho::VarChirho(mfix_m_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::fun_chirho(
                TyChirho::VarChirho(mfix_a_chirho),
                mfix_result_chirho.clone(),
            ),
            mfix_result_chirho,
        ),
    };
    env_chirho.bind_chirho("mfix".to_string(), mfix_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.Fix.mfix".to_string(), mfix_scheme_chirho);

    // (>>=) :: forall m a b. Monad m => m a -> (a -> m b) -> m b
    let bind_m_chirho = TyVarChirho(1800);
    let bind_a_chirho = TyVarChirho(1801);
    let bind_b_chirho = TyVarChirho(1802);
    env_chirho.bind_chirho(
        ">>=".to_string(),
        SchemeChirho {
            vars_chirho: vec![bind_m_chirho, bind_a_chirho, bind_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monad".to_string(),
                ty_chirho: TyChirho::VarChirho(bind_m_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(bind_m_chirho)),
                        Box::new(TyChirho::VarChirho(bind_a_chirho)),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(bind_a_chirho),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(bind_m_chirho)),
                            Box::new(TyChirho::VarChirho(bind_b_chirho)),
                        ),
                    ),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(bind_m_chirho)),
                    Box::new(TyChirho::VarChirho(bind_b_chirho)),
                ),
            ),
        },
    );

    // (>>) :: forall m a b. Monad m => m a -> m b -> m b
    let then_m_chirho = TyVarChirho(1900);
    let then_a_chirho = TyVarChirho(1901);
    let then_b_chirho = TyVarChirho(1902);
    env_chirho.bind_chirho(
        ">>".to_string(),
        SchemeChirho {
            vars_chirho: vec![then_m_chirho, then_a_chirho, then_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monad".to_string(),
                ty_chirho: TyChirho::VarChirho(then_m_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(then_m_chirho)),
                        Box::new(TyChirho::VarChirho(then_a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(then_m_chirho)),
                        Box::new(TyChirho::VarChirho(then_b_chirho)),
                    ),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(then_m_chirho)),
                    Box::new(TyChirho::VarChirho(then_b_chirho)),
                ),
            ),
        },
    );

    // >=> :: Monad m => (a -> m b) -> (b -> m c) -> a -> m c
    {
        let kleisli_m_chirho = TyVarChirho(1903);
        let kleisli_a_chirho = TyVarChirho(1904);
        let kleisli_b_chirho = TyVarChirho(1905);
        let kleisli_c_chirho = TyVarChirho(1906);
        env_chirho.bind_chirho(
            ">=>".to_string(),
            SchemeChirho {
                vars_chirho: vec![
                    kleisli_m_chirho,
                    kleisli_a_chirho,
                    kleisli_b_chirho,
                    kleisli_c_chirho,
                ],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(kleisli_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        // a -> m b
                        TyChirho::FunChirho(
                            Box::new(TyChirho::VarChirho(kleisli_a_chirho)),
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(kleisli_m_chirho)),
                                Box::new(TyChirho::VarChirho(kleisli_b_chirho)),
                            )),
                            MultChirho::ManyChirho,
                        ),
                        // b -> m c
                        TyChirho::FunChirho(
                            Box::new(TyChirho::VarChirho(kleisli_b_chirho)),
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(kleisli_m_chirho)),
                                Box::new(TyChirho::VarChirho(kleisli_c_chirho)),
                            )),
                            MultChirho::ManyChirho,
                        ),
                        // a
                        TyChirho::VarChirho(kleisli_a_chirho),
                    ],
                    // m c
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(kleisli_m_chirho)),
                        Box::new(TyChirho::VarChirho(kleisli_c_chirho)),
                    ),
                ),
            },
        );
    }
    // <=< :: Monad m => (b -> m c) -> (a -> m b) -> a -> m c
    {
        let kleisli_m_chirho = TyVarChirho(1907);
        let kleisli_a_chirho = TyVarChirho(1908);
        let kleisli_b_chirho = TyVarChirho(1909);
        let kleisli_c_chirho = TyVarChirho(1910);
        env_chirho.bind_chirho(
            "<=<".to_string(),
            SchemeChirho {
                vars_chirho: vec![
                    kleisli_m_chirho,
                    kleisli_a_chirho,
                    kleisli_b_chirho,
                    kleisli_c_chirho,
                ],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(kleisli_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        // b -> m c
                        TyChirho::FunChirho(
                            Box::new(TyChirho::VarChirho(kleisli_b_chirho)),
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(kleisli_m_chirho)),
                                Box::new(TyChirho::VarChirho(kleisli_c_chirho)),
                            )),
                            MultChirho::ManyChirho,
                        ),
                        // a -> m b
                        TyChirho::FunChirho(
                            Box::new(TyChirho::VarChirho(kleisli_a_chirho)),
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(kleisli_m_chirho)),
                                Box::new(TyChirho::VarChirho(kleisli_b_chirho)),
                            )),
                            MultChirho::ManyChirho,
                        ),
                        // a
                        TyChirho::VarChirho(kleisli_a_chirho),
                    ],
                    // m c
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(kleisli_m_chirho)),
                        Box::new(TyChirho::VarChirho(kleisli_c_chirho)),
                    ),
                ),
            },
        );
    }

    // getLine :: IO String
    env_chirho.bind_chirho(
        "getLine".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::string_chirho())),
    );

    // getChar :: IO Char
    env_chirho.bind_chirho(
        "getChar".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::char_chirho())),
    );

    // getContents :: IO String — read all stdin as a single lazy String
    env_chirho.bind_chirho(
        "getContents".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::string_chirho())),
    );

    // readFile :: String -> IO String
    env_chirho.bind_chirho(
        "readFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(TyChirho::string_chirho()),
        )),
    );

    // writeFile :: String -> String -> IO ()
    env_chirho.bind_chirho(
        "writeFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), TyChirho::string_chirho()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // appendFile :: String -> String -> IO ()
    env_chirho.bind_chirho(
        "appendFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), TyChirho::string_chirho()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // interact :: (String -> String) -> IO ()
    env_chirho.bind_chirho(
        "interact".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // throwErrnoIfMinus1_ :: String -> IO a -> IO ()
    {
        let errno_a_chirho = TyVarChirho(1694);
        env_chirho.bind_chirho(
            "throwErrnoIfMinus1_".to_string(),
            SchemeChirho {
                vars_chirho: vec![errno_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::string_chirho(),
                        TyChirho::io_chirho(TyChirho::VarChirho(errno_a_chirho)),
                    ],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // newIORef :: a -> IO (IORef a)  (IORef a ≈ Int at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3200));
        env_chirho.bind_chirho(
            "newIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3200)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho,
                    TyChirho::io_chirho(TyChirho::int_chirho()),
                ),
            },
        );
    }

    // readIORef :: IORef a -> IO a  (IORef a ≈ Int at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3201));
        env_chirho.bind_chirho(
            "readIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3201)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::int_chirho(),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // writeIORef :: IORef a -> a -> IO ()
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3202));
        env_chirho.bind_chirho(
            "writeIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3202)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::int_chirho(), a_chirho],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // modifyIORef :: IORef a -> (a -> a) -> IO ()
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3203));
        env_chirho.bind_chirho(
            "modifyIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3203)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::int_chirho(),
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    ],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // ── Control.Monad.ST operations ──
    // Keep the GHC-shaped surface while unification bridges saturated ST uses
    // to the current first-order runtime representation.
    // newSTRef :: a -> ST s (STRef s a)
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3279));
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3280));
        let stref_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(s_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        let st_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(s_chirho),
            )),
            Box::new(stref_ty_chirho),
        );
        env_chirho.bind_chirho(
            "newSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3279), TyVarChirho(3280)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(a_chirho, st_ty_chirho),
            },
        );
    }

    // readSTRef :: STRef s a -> ST s a
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3285));
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3281));
        let stref_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(s_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        let st_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(s_chirho),
            )),
            Box::new(a_chirho),
        );
        env_chirho.bind_chirho(
            "readSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3285), TyVarChirho(3281)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(stref_ty_chirho, st_ty_chirho),
            },
        );
    }

    // writeSTRef :: STRef s a -> a -> ST s ()
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3286));
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3282));
        let stref_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(s_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        let st_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(s_chirho),
            )),
            Box::new(TyChirho::unit_chirho()),
        );
        env_chirho.bind_chirho(
            "writeSTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3286), TyVarChirho(3282)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(vec![stref_ty_chirho, a_chirho], st_ty_chirho),
            },
        );
    }

    // modifySTRef :: STRef s a -> (a -> a) -> ST s ()
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3287));
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3284));
        let stref_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("STRef".to_string())),
                Box::new(s_chirho.clone()),
            )),
            Box::new(a_chirho.clone()),
        );
        let st_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(s_chirho),
            )),
            Box::new(TyChirho::unit_chirho()),
        );
        env_chirho.bind_chirho(
            "modifySTRef".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3287), TyVarChirho(3284)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        stref_ty_chirho,
                        TyChirho::fun_chirho(a_chirho.clone(), a_chirho),
                    ],
                    st_ty_chirho,
                ),
            },
        );
    }

    // runST :: forall a. (forall s. ST s a) -> a
    {
        let s_var_chirho = TyVarChirho(3288);
        let a_var_chirho = TyVarChirho(3283);
        let s_chirho = TyChirho::VarChirho(s_var_chirho);
        let a_chirho = TyChirho::VarChirho(a_var_chirho);
        let st_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(s_chirho),
            )),
            Box::new(a_chirho.clone()),
        );
        env_chirho.bind_chirho(
            "runST".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_var_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ForallChirho {
                        vars_chirho: vec![s_var_chirho],
                        body_chirho: Box::new(st_ty_chirho),
                    },
                    a_chirho,
                ),
            },
        );
    }
}
