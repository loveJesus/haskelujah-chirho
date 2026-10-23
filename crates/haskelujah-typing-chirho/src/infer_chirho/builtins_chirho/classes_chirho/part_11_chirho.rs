// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 11 of 14: original lines 17271-18233 of
//! `seed_builtins_chirho`, in order (32 named binds).

use super::super::transformer_types_chirho::mk_maybe_t_ty_chirho;
use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_11_chirho(env_chirho: &mut TyEnvChirho) {
    // -----------------------------------------------------------------------
    // ExceptT monad transformer
    // -----------------------------------------------------------------------

    // ExceptT :: m (Either e a) -> ExceptT e m a
    // runExceptT :: ExceptT e m a -> m (Either e a)
    {
        let et_e_chirho = TyVarChirho(3550);
        let et_a_chirho = TyVarChirho(3551);
        let et_m_chirho = TyVarChirho(3552);
        let either_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(TyChirho::VarChirho(et_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(et_a_chirho)),
        );
        // m (Either e a)
        let m_either_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(et_m_chirho)),
            Box::new(either_ea_chirho.clone()),
        );
        // ExceptT e m a
        let exceptt_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(et_e_chirho)),
                )),
                Box::new(TyChirho::VarChirho(et_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(et_a_chirho)),
        );
        env_chirho.bind_chirho(
            "ExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![et_e_chirho, et_m_chirho, et_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    m_either_ea_chirho.clone(),
                    exceptt_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "runExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![et_e_chirho, et_m_chirho, et_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(exceptt_ty_chirho, m_either_ea_chirho),
            },
        );
    }

    // throwE :: e -> ExceptT e m a
    {
        let te_e_chirho = TyVarChirho(3552);
        let te_m_chirho = TyVarChirho(3553);
        let te_a_chirho = TyVarChirho(3554);
        let exceptt_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(te_e_chirho)),
                )),
                Box::new(TyChirho::VarChirho(te_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(te_a_chirho)),
        );
        env_chirho.bind_chirho(
            "throwE".to_string(),
            SchemeChirho {
                vars_chirho: vec![te_e_chirho, te_m_chirho, te_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(te_e_chirho),
                    exceptt_ty_chirho,
                ),
            },
        );
    }

    // returnExceptT :: a -> ExceptT e m a
    {
        let re_e_chirho = TyVarChirho(3555);
        let re_m_chirho = TyVarChirho(3556);
        let re_a_chirho = TyVarChirho(3557);
        let exceptt_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(re_e_chirho)),
                )),
                Box::new(TyChirho::VarChirho(re_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(re_a_chirho)),
        );
        env_chirho.bind_chirho(
            "returnExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![re_e_chirho, re_m_chirho, re_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(re_a_chirho),
                    exceptt_ty_chirho,
                ),
            },
        );
    }

    // bindExceptT :: ExceptT e m a -> (a -> ExceptT e m b) -> ExceptT e m b
    {
        let be_e_chirho = TyVarChirho(3558);
        let be_m_chirho = TyVarChirho(3559);
        let be_a_chirho = TyVarChirho(3560);
        let be_b_chirho = TyVarChirho(3561);
        let exceptt_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(be_e_chirho)),
                )),
                Box::new(TyChirho::VarChirho(be_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(be_a_chirho)),
        );
        let exceptt_eb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(be_e_chirho)),
                )),
                Box::new(TyChirho::VarChirho(be_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(be_b_chirho)),
        );
        env_chirho.bind_chirho(
            "bindExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![be_e_chirho, be_m_chirho, be_a_chirho, be_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    exceptt_ea_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(be_a_chirho),
                            exceptt_eb_chirho.clone(),
                        ),
                        exceptt_eb_chirho,
                    ),
                ),
            },
        );
    }

    // catchE :: ExceptT e m a -> (e -> ExceptT e' m a) -> ExceptT e' m a
    {
        let ce_e_chirho = TyVarChirho(3562);
        let ce_e_prime_chirho = TyVarChirho(3563);
        let ce_m_chirho = TyVarChirho(3564);
        let ce_a_chirho = TyVarChirho(3565);
        let exceptt_ea_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(ce_e_chirho)),
                )),
                Box::new(TyChirho::VarChirho(ce_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ce_a_chirho)),
        );
        let exceptt_e_prime_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ExceptT".to_string())),
                    Box::new(TyChirho::VarChirho(ce_e_prime_chirho)),
                )),
                Box::new(TyChirho::VarChirho(ce_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ce_a_chirho)),
        );
        env_chirho.bind_chirho(
            "catchE".to_string(),
            SchemeChirho {
                vars_chirho: vec![ce_e_chirho, ce_e_prime_chirho, ce_m_chirho, ce_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    exceptt_ea_chirho.clone(),
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(ce_e_chirho),
                            exceptt_e_prime_a_chirho.clone(),
                        ),
                        exceptt_e_prime_a_chirho,
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // MaybeT operations (returnMaybeT, bindMaybeT)
    // -----------------------------------------------------------------------

    // returnMaybeT :: forall m a. Monad m => a -> MaybeT m a
    {
        let rmt_m_chirho = TyVarChirho(3570);
        let rmt_a_chirho = TyVarChirho(3571);
        let maybet_a_chirho = mk_maybe_t_ty_chirho(
            TyChirho::VarChirho(rmt_m_chirho),
            TyChirho::VarChirho(rmt_a_chirho),
        );
        env_chirho.bind_chirho(
            "returnMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rmt_m_chirho, rmt_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(rmt_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(rmt_a_chirho), maybet_a_chirho),
            },
        );
    }

    // bindMaybeT :: forall m a b. Monad m => MaybeT m a -> (a -> MaybeT m b) -> MaybeT m b
    {
        let bmt_m_chirho = TyVarChirho(3572);
        let bmt_a_chirho = TyVarChirho(3573);
        let bmt_b_chirho = TyVarChirho(3574);
        let maybet_a_chirho = mk_maybe_t_ty_chirho(
            TyChirho::VarChirho(bmt_m_chirho),
            TyChirho::VarChirho(bmt_a_chirho),
        );
        let maybet_b_chirho = mk_maybe_t_ty_chirho(
            TyChirho::VarChirho(bmt_m_chirho),
            TyChirho::VarChirho(bmt_b_chirho),
        );
        env_chirho.bind_chirho(
            "bindMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![bmt_m_chirho, bmt_a_chirho, bmt_b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(bmt_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    maybet_a_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bmt_a_chirho),
                            maybet_b_chirho.clone(),
                        ),
                        maybet_b_chirho,
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // WriterT operations (WriterT, runWriterT, runWriter, tell, returnWriterT,
    //   bindWriterT, execWriterT, execWriter)
    // -----------------------------------------------------------------------

    // WriterT :: forall w m a. (a, w) -> WriterT w m a
    {
        let wt_w_chirho = TyVarChirho(3580);
        let wt_m_chirho = TyVarChirho(3581);
        let wt_a_chirho = TyVarChirho(3582);
        let writert_wma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(wt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(wt_m_chirho)),
        );
        let tuple_aw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(wt_a_chirho),
            TyChirho::VarChirho(wt_w_chirho),
        ]);
        env_chirho.bind_chirho(
            "WriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![wt_w_chirho, wt_m_chirho, wt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    tuple_aw_chirho,
                    TyChirho::AppChirho(
                        Box::new(writert_wma_chirho),
                        Box::new(TyChirho::VarChirho(wt_a_chirho)),
                    ),
                ),
            },
        );
    }

    // runWriterT :: forall w m a. WriterT w m a -> (a, w)
    {
        let rwt_w_chirho = TyVarChirho(3583);
        let rwt_m_chirho = TyVarChirho(3584);
        let rwt_a_chirho = TyVarChirho(3585);
        let writert_wma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("WriterT".to_string())),
                Box::new(TyChirho::VarChirho(rwt_w_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rwt_m_chirho)),
        );
        let tuple_aw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(rwt_a_chirho),
            TyChirho::VarChirho(rwt_w_chirho),
        ]);
        env_chirho.bind_chirho(
            "runWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rwt_w_chirho, rwt_m_chirho, rwt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(writert_wma_chirho),
                        Box::new(TyChirho::VarChirho(rwt_a_chirho)),
                    ),
                    tuple_aw_chirho,
                ),
            },
        );
    }

    // runWriter :: forall w a. WriterT w Identity a -> (a, w)  (alias)
    {
        let rw_w_chirho = TyVarChirho(3586);
        let rw_a_chirho = TyVarChirho(3587);
        let writer_identity_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(rw_w_chirho)),
                )),
                Box::new(TyChirho::ConChirho("Identity".to_string())),
            )),
            Box::new(TyChirho::VarChirho(rw_a_chirho)),
        );
        let tuple_aw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(rw_a_chirho),
            TyChirho::VarChirho(rw_w_chirho),
        ]);
        env_chirho.bind_chirho(
            "runWriter".to_string(),
            SchemeChirho {
                vars_chirho: vec![rw_w_chirho, rw_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(writer_identity_wa_chirho, tuple_aw_chirho),
            },
        );
    }

    // tell :: forall w m. Monad m => w -> WriterT w m ()
    {
        let t_w_chirho = TyVarChirho(3588);
        let t_m_chirho = TyVarChirho(3589);
        let writert_wm_unit_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(t_w_chirho)),
                )),
                Box::new(TyChirho::VarChirho(t_m_chirho)),
            )),
            Box::new(TyChirho::unit_chirho()),
        );
        env_chirho.bind_chirho(
            "tell".to_string(),
            SchemeChirho {
                vars_chirho: vec![t_w_chirho, t_m_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(t_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(t_w_chirho),
                    writert_wm_unit_chirho,
                ),
            },
        );
    }

    // returnWriterT :: forall w m a. Monad m => a -> WriterT w m a
    {
        let rwt_w_chirho = TyVarChirho(3590);
        let rwt_m_chirho = TyVarChirho(3591);
        let rwt_a_chirho = TyVarChirho(3592);
        let writert_wma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(rwt_w_chirho)),
                )),
                Box::new(TyChirho::VarChirho(rwt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rwt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "returnWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rwt_w_chirho, rwt_m_chirho, rwt_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(rwt_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rwt_a_chirho),
                    writert_wma_chirho,
                ),
            },
        );
    }

    // bindWriterT :: forall w m a b. Monad m =>
    //   WriterT w m a -> (a -> WriterT w m b) -> WriterT w m b
    {
        let bwt_w_chirho = TyVarChirho(3593);
        let bwt_m_chirho = TyVarChirho(3594);
        let bwt_a_chirho = TyVarChirho(3595);
        let bwt_b_chirho = TyVarChirho(3596);
        let writert_wma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(bwt_w_chirho)),
                )),
                Box::new(TyChirho::VarChirho(bwt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bwt_a_chirho)),
        );
        let writert_wmb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(bwt_w_chirho)),
                )),
                Box::new(TyChirho::VarChirho(bwt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(bwt_b_chirho)),
        );
        env_chirho.bind_chirho(
            "bindWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![bwt_w_chirho, bwt_m_chirho, bwt_a_chirho, bwt_b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(bwt_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    writert_wma_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bwt_a_chirho),
                            writert_wmb_chirho.clone(),
                        ),
                        writert_wmb_chirho,
                    ),
                ),
            },
        );
    }

    // execWriterT :: forall w m a. Monad m => WriterT w m a -> w
    {
        let ewt_w_chirho = TyVarChirho(3597);
        let ewt_m_chirho = TyVarChirho(3598);
        let ewt_a_chirho = TyVarChirho(3599);
        let writert_wma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(ewt_w_chirho)),
                )),
                Box::new(TyChirho::VarChirho(ewt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(ewt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "execWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![ewt_w_chirho, ewt_m_chirho, ewt_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(ewt_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    writert_wma_chirho,
                    TyChirho::VarChirho(ewt_w_chirho),
                ),
            },
        );
    }

    // execWriter :: forall w a. WriterT w Identity a -> w
    {
        let ew_w_chirho = TyVarChirho(3600);
        let ew_a_chirho = TyVarChirho(3601);
        let writer_identity_wa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("WriterT".to_string())),
                    Box::new(TyChirho::VarChirho(ew_w_chirho)),
                )),
                Box::new(TyChirho::ConChirho("Identity".to_string())),
            )),
            Box::new(TyChirho::VarChirho(ew_a_chirho)),
        );
        env_chirho.bind_chirho(
            "execWriter".to_string(),
            SchemeChirho {
                vars_chirho: vec![ew_w_chirho, ew_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    writer_identity_wa_chirho,
                    TyChirho::VarChirho(ew_w_chirho),
                ),
            },
        );
    }

    // RWST :: forall r w s m a. (r -> s -> m (a, s, w)) -> RWST r w s m a
    // runRWST :: forall r w s m a. RWST r w s m a -> r -> s -> m (a, s, w)
    {
        let rwst_r_chirho = TyVarChirho(3602);
        let rwst_w_chirho = TyVarChirho(3603);
        let rwst_s_chirho = TyVarChirho(3604);
        let rwst_m_chirho = TyVarChirho(3605);
        let rwst_a_chirho = TyVarChirho(3606);
        let tuple_asw_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(rwst_a_chirho),
            TyChirho::VarChirho(rwst_s_chirho),
            TyChirho::VarChirho(rwst_w_chirho),
        ]);
        let m_tuple_asw_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(rwst_m_chirho)),
            Box::new(tuple_asw_chirho.clone()),
        );
        let rwst_rwsm_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("RWST".to_string())),
                        Box::new(TyChirho::VarChirho(rwst_r_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(rwst_w_chirho)),
                )),
                Box::new(TyChirho::VarChirho(rwst_s_chirho)),
            )),
            Box::new(TyChirho::VarChirho(rwst_m_chirho)),
        );
        let rwst_full_chirho = TyChirho::AppChirho(
            Box::new(rwst_rwsm_chirho.clone()),
            Box::new(TyChirho::VarChirho(rwst_a_chirho)),
        );
        let runner_chirho = TyChirho::fun_chirho(
            TyChirho::VarChirho(rwst_r_chirho),
            TyChirho::fun_chirho(
                TyChirho::VarChirho(rwst_s_chirho),
                m_tuple_asw_chirho.clone(),
            ),
        );
        env_chirho.bind_chirho(
            "RWST".to_string(),
            SchemeChirho {
                vars_chirho: vec![
                    rwst_r_chirho,
                    rwst_w_chirho,
                    rwst_s_chirho,
                    rwst_m_chirho,
                    rwst_a_chirho,
                ],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(runner_chirho.clone(), rwst_full_chirho.clone()),
            },
        );
        env_chirho.bind_chirho(
            "runRWST".to_string(),
            SchemeChirho {
                vars_chirho: vec![
                    rwst_r_chirho,
                    rwst_w_chirho,
                    rwst_s_chirho,
                    rwst_m_chirho,
                    rwst_a_chirho,
                ],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(rwst_full_chirho, runner_chirho),
            },
        );
    }

    // -----------------------------------------------------------------------
    // Additional utility functions (repeat, cycle, fix, group, etc.)
    // -----------------------------------------------------------------------

    // repeat :: forall a. a -> [a]
    {
        let rep_a_chirho = TyVarChirho(4100);
        env_chirho.bind_chirho(
            "repeat".to_string(),
            SchemeChirho {
                vars_chirho: vec![rep_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(rep_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(rep_a_chirho))),
                ),
            },
        );
    }

    // cycle :: forall a. [a] -> [a]
    {
        let cyc_a_chirho = TyVarChirho(4101);
        env_chirho.bind_chirho(
            "cycle".to_string(),
            SchemeChirho {
                vars_chirho: vec![cyc_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cyc_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cyc_a_chirho))),
                ),
            },
        );
    }

    // fix :: forall a. (a -> a) -> a
    {
        let fix_a_chirho = TyVarChirho(4102);
        env_chirho.bind_chirho(
            "fix".to_string(),
            SchemeChirho {
                vars_chirho: vec![fix_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(fix_a_chirho),
                        TyChirho::VarChirho(fix_a_chirho),
                    ),
                    TyChirho::VarChirho(fix_a_chirho),
                ),
            },
        );
    }

    // group :: [Int] -> [[Int]]
    env_chirho.bind_chirho(
        "group".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                    TyChirho::int_chirho(),
                )))),
            ),
        },
    );

    // zipWith3 :: forall a b c d. (a -> b -> c -> d) -> [a] -> [b] -> [c] -> [d]
    {
        let zw3_a_chirho = TyVarChirho(4110);
        let zw3_b_chirho = TyVarChirho(4111);
        let zw3_c_chirho = TyVarChirho(4112);
        let zw3_d_chirho = TyVarChirho(4113);
        env_chirho.bind_chirho(
            "zipWith3".to_string(),
            SchemeChirho {
                vars_chirho: vec![zw3_a_chirho, zw3_b_chirho, zw3_c_chirho, zw3_d_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(zw3_a_chirho),
                                TyChirho::VarChirho(zw3_b_chirho),
                                TyChirho::VarChirho(zw3_c_chirho),
                            ],
                            TyChirho::VarChirho(zw3_d_chirho),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_b_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_c_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zw3_d_chirho))),
                ),
            },
        );
    }

    // first :: Bifunctor p => (a -> b) -> p a c -> p b c
    {
        let fi_p_chirho = TyVarChirho(4120);
        let fi_a_chirho = TyVarChirho(4121);
        let fi_b_chirho = TyVarChirho(4122);
        let fi_c_chirho = TyVarChirho(4126);
        env_chirho.bind_chirho(
            "first".to_string(),
            SchemeChirho {
                vars_chirho: vec![fi_p_chirho, fi_a_chirho, fi_b_chirho, fi_c_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Bifunctor".to_string(),
                    ty_chirho: TyChirho::VarChirho(fi_p_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(fi_a_chirho),
                            TyChirho::VarChirho(fi_b_chirho),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(fi_p_chirho)),
                                Box::new(TyChirho::VarChirho(fi_a_chirho)),
                            )),
                            Box::new(TyChirho::VarChirho(fi_c_chirho)),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(fi_p_chirho)),
                            Box::new(TyChirho::VarChirho(fi_b_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(fi_c_chirho)),
                    ),
                ),
            },
        );
    }

    // second :: Bifunctor p => (b -> c) -> p a b -> p a c
    {
        let se_p_chirho = TyVarChirho(4123);
        let se_a_chirho = TyVarChirho(4124);
        let se_b_chirho = TyVarChirho(4125);
        let se_c_chirho = TyVarChirho(4127);
        env_chirho.bind_chirho(
            "second".to_string(),
            SchemeChirho {
                vars_chirho: vec![se_p_chirho, se_a_chirho, se_b_chirho, se_c_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Bifunctor".to_string(),
                    ty_chirho: TyChirho::VarChirho(se_p_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(se_b_chirho),
                            TyChirho::VarChirho(se_c_chirho),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(se_p_chirho)),
                                Box::new(TyChirho::VarChirho(se_a_chirho)),
                            )),
                            Box::new(TyChirho::VarChirho(se_b_chirho)),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(se_p_chirho)),
                            Box::new(TyChirho::VarChirho(se_a_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(se_c_chirho)),
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // Exception handling
    // -----------------------------------------------------------------------

    // catch :: forall e a. Exception e => IO a -> (e -> IO a) -> IO a
    {
        let catch_e_chirho = TyVarChirho(4129);
        let catch_a_chirho = TyVarChirho(4130);
        env_chirho.bind_chirho(
            "catch".to_string(),
            SchemeChirho {
                vars_chirho: vec![catch_e_chirho, catch_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Exception".to_string(),
                    ty_chirho: TyChirho::VarChirho(catch_e_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(TyChirho::VarChirho(catch_a_chirho)),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(catch_e_chirho),
                            TyChirho::io_chirho(TyChirho::VarChirho(catch_a_chirho)),
                        ),
                    ],
                    TyChirho::io_chirho(TyChirho::VarChirho(catch_a_chirho)),
                ),
            },
        );
    }

    // throw :: forall e a. Exception e => e -> a
    {
        let throw_e_chirho = TyVarChirho(4130);
        let throw_a_chirho = TyVarChirho(4131);
        env_chirho.bind_chirho(
            "throw".to_string(),
            SchemeChirho {
                vars_chirho: vec![throw_e_chirho, throw_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Exception".to_string(),
                    ty_chirho: TyChirho::VarChirho(throw_e_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(throw_e_chirho),
                    TyChirho::VarChirho(throw_a_chirho),
                ),
            },
        );
    }

    // try :: forall e a. Exception e => IO a -> IO (Either e a)
    {
        let try_e_chirho = TyVarChirho(4133);
        let try_a_chirho = TyVarChirho(4132);
        // Either e a = App (App (Con "Either") e) (Var a)
        let either_e_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(TyChirho::VarChirho(try_e_chirho)),
            )),
            Box::new(TyChirho::VarChirho(try_a_chirho)),
        );
        env_chirho.bind_chirho(
            "try".to_string(),
            SchemeChirho {
                vars_chirho: vec![try_e_chirho, try_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Exception".to_string(),
                    ty_chirho: TyChirho::VarChirho(try_e_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(TyChirho::VarChirho(try_a_chirho)),
                    TyChirho::io_chirho(either_e_a_chirho),
                ),
            },
        );
    }

    // throwIO :: forall e a. Exception e => e -> IO a
    {
        let throwio_e_chirho = TyVarChirho(4159);
        let throwio_a_chirho = TyVarChirho(4160);
        env_chirho.bind_chirho(
            "throwIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![throwio_e_chirho, throwio_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Exception".to_string(),
                    ty_chirho: TyChirho::VarChirho(throwio_e_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(throwio_e_chirho),
                    TyChirho::io_chirho(TyChirho::VarChirho(throwio_a_chirho)),
                ),
            },
        );
    }

    // bracket :: forall a b c. IO a -> (a -> IO b) -> (a -> IO c) -> IO c
    {
        let br_a_chirho = TyVarChirho(4161);
        let br_b_chirho = TyVarChirho(4162);
        let br_c_chirho = TyVarChirho(4163);
        env_chirho.bind_chirho(
            "bracket".to_string(),
            SchemeChirho {
                vars_chirho: vec![br_a_chirho, br_b_chirho, br_c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(TyChirho::VarChirho(br_a_chirho)),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(br_a_chirho),
                            TyChirho::io_chirho(TyChirho::VarChirho(br_b_chirho)),
                        ),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(br_a_chirho),
                            TyChirho::io_chirho(TyChirho::VarChirho(br_c_chirho)),
                        ),
                    ],
                    TyChirho::io_chirho(TyChirho::VarChirho(br_c_chirho)),
                ),
            },
        );
    }

    // finally :: forall a b. IO a -> IO b -> IO a
    {
        let fin_a_chirho = TyVarChirho(4164);
        let fin_b_chirho = TyVarChirho(4165);
        env_chirho.bind_chirho(
            "finally".to_string(),
            SchemeChirho {
                vars_chirho: vec![fin_a_chirho, fin_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(TyChirho::VarChirho(fin_a_chirho)),
                        TyChirho::io_chirho(TyChirho::VarChirho(fin_b_chirho)),
                    ],
                    TyChirho::io_chirho(TyChirho::VarChirho(fin_a_chirho)),
                ),
            },
        );
    }

    // both :: forall a b. (a -> b) -> (a, a) -> (b, b)
    {
        let bo_a_chirho = TyVarChirho(4126);
        let bo_b_chirho = TyVarChirho(4127);
        env_chirho.bind_chirho(
            "both".to_string(),
            SchemeChirho {
                vars_chirho: vec![bo_a_chirho, bo_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bo_a_chirho),
                            TyChirho::VarChirho(bo_b_chirho),
                        ),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(bo_a_chirho),
                            TyChirho::VarChirho(bo_a_chirho),
                        ]),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(bo_b_chirho),
                        TyChirho::VarChirho(bo_b_chirho),
                    ]),
                ),
            },
        );
    }
}
