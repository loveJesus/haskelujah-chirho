// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 06 of 14: original lines 12544-13473 of
//! `seed_builtins_chirho`, in order (52 named binds).

use super::super::transformer_types_chirho::mk_state_t_ty_chirho;
use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_06_chirho(env_chirho: &mut TyEnvChirho) {
    // ── STM (Software Transactional Memory) operations ──
    let mk_tvar_ty_chirho = |inner_ty_chirho: TyChirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("TVar".to_string())),
            Box::new(inner_ty_chirho),
        )
    };
    let mk_stm_ty_chirho = |inner_ty_chirho: TyChirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("STM".to_string())),
            Box::new(inner_ty_chirho),
        )
    };

    // newTVar :: a -> STM (TVar a)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3290));
        env_chirho.bind_chirho(
            "newTVar".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3290)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho.clone(),
                    mk_stm_ty_chirho(mk_tvar_ty_chirho(a_chirho)),
                ),
            },
        );
    }

    // newTVarIO :: a -> IO (TVar a)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3291));
        env_chirho.bind_chirho(
            "newTVarIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3291)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    a_chirho.clone(),
                    TyChirho::io_chirho(mk_tvar_ty_chirho(a_chirho)),
                ),
            },
        );
    }

    // readTVar :: TVar a -> STM a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3292));
        env_chirho.bind_chirho(
            "readTVar".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3292)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    mk_tvar_ty_chirho(a_chirho.clone()),
                    mk_stm_ty_chirho(a_chirho),
                ),
            },
        );
    }

    // readTVarIO :: TVar a -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3293));
        env_chirho.bind_chirho(
            "readTVarIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3293)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    mk_tvar_ty_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // writeTVar :: TVar a -> a -> STM ()
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3294));
        env_chirho.bind_chirho(
            "writeTVar".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3294)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![mk_tvar_ty_chirho(a_chirho.clone()), a_chirho],
                    mk_stm_ty_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // atomically :: STM a -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3295));
        env_chirho.bind_chirho(
            "atomically".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3295)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    mk_stm_ty_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // retry :: STM a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3296));
        env_chirho.bind_chirho(
            "retry".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3296)],
                preds_chirho: vec![],
                ty_chirho: mk_stm_ty_chirho(a_chirho),
            },
        );
    }

    // orElse :: STM a -> STM a -> STM a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3297));
        env_chirho.bind_chirho(
            "orElse".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3297)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        mk_stm_ty_chirho(a_chirho.clone()),
                        mk_stm_ty_chirho(a_chirho.clone()),
                    ],
                    mk_stm_ty_chirho(a_chirho),
                ),
            },
        );
    }

    // GHC.Event timer manager helpers
    {
        let timer_manager_ty_chirho = TyChirho::ConChirho("TimerManager".to_string());
        let timeout_key_ty_chirho = TyChirho::ConChirho("TimeoutKey".to_string());

        // getSystemTimerManager :: IO TimerManager
        env_chirho.bind_chirho(
            "getSystemTimerManager".to_string(),
            SchemeChirho {
                vars_chirho: vec![],
                preds_chirho: vec![],
                ty_chirho: TyChirho::io_chirho(timer_manager_ty_chirho.clone()),
            },
        );

        // registerTimeout :: TimerManager -> Int -> IO () -> IO TimeoutKey
        env_chirho.bind_chirho(
            "registerTimeout".to_string(),
            SchemeChirho {
                vars_chirho: vec![],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        timer_manager_ty_chirho.clone(),
                        TyChirho::int_chirho(),
                        TyChirho::io_chirho(TyChirho::unit_chirho()),
                    ],
                    TyChirho::io_chirho(timeout_key_ty_chirho.clone()),
                ),
            },
        );

        // unregisterTimeout :: TimerManager -> TimeoutKey -> IO ()
        env_chirho.bind_chirho(
            "unregisterTimeout".to_string(),
            SchemeChirho {
                vars_chirho: vec![],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![timer_manager_ty_chirho, timeout_key_ty_chirho],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // ── Monad Transformer operations ──

    // runStateT :: StateT s m a -> s -> m (a, s)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3400));
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3401));
        let m_chirho = TyChirho::VarChirho(TyVarChirho(3409));
        let state_t_ty_chirho =
            mk_state_t_ty_chirho(s_chirho.clone(), m_chirho.clone(), a_chirho.clone());
        env_chirho.bind_chirho(
            "runStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3400), TyVarChirho(3401), TyVarChirho(3409)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![state_t_ty_chirho, s_chirho.clone()],
                    TyChirho::AppChirho(
                        Box::new(m_chirho),
                        Box::new(TyChirho::TupleChirho(vec![a_chirho, s_chirho])),
                    ),
                ),
            },
        );
    }

    // evalStateT :: StateT s m a -> s -> m a
    for name_chirho in &["evalStateT"] {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3402));
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3403));
        let m_chirho = TyChirho::VarChirho(TyVarChirho(3415));
        let state_t_ty_chirho =
            mk_state_t_ty_chirho(s_chirho.clone(), m_chirho.clone(), a_chirho.clone());
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3402), TyVarChirho(3403), TyVarChirho(3415)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![state_t_ty_chirho, s_chirho],
                    TyChirho::AppChirho(Box::new(m_chirho), Box::new(a_chirho)),
                ),
            },
        );
    }

    // execStateT :: StateT s m a -> s -> m s
    for name_chirho in &["execStateT"] {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3404));
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3405));
        let m_chirho = TyChirho::VarChirho(TyVarChirho(3416));
        let state_t_ty_chirho = mk_state_t_ty_chirho(s_chirho.clone(), m_chirho.clone(), a_chirho);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3404), TyVarChirho(3405), TyVarChirho(3416)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![state_t_ty_chirho, s_chirho.clone()],
                    TyChirho::AppChirho(Box::new(m_chirho), Box::new(s_chirho)),
                ),
            },
        );
    }

    // get :: IO s
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3406));
        env_chirho.bind_chirho(
            "get".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3406)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::io_chirho(s_chirho),
            },
        );
    }

    // put :: s -> IO ()
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3407));
        env_chirho.bind_chirho(
            "put".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3407)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    s_chirho,
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // modify :: (s -> s) -> IO ()
    {
        let s_chirho = TyChirho::VarChirho(TyVarChirho(3408));
        env_chirho.bind_chirho(
            "modify".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3408)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(s_chirho.clone(), s_chirho),
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // runReaderT :: (IO a) -> r -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3410));
        let r_chirho = TyChirho::VarChirho(TyVarChirho(3411));
        env_chirho.bind_chirho(
            "runReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3410), TyVarChirho(3411)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::io_chirho(a_chirho.clone()), r_chirho],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // ask :: IO r
    {
        let r_chirho = TyChirho::VarChirho(TyVarChirho(3412));
        env_chirho.bind_chirho(
            "ask".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3412)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::io_chirho(r_chirho),
            },
        );
    }

    // local :: (r -> r) -> IO a -> IO a
    {
        let r_chirho = TyChirho::VarChirho(TyVarChirho(3413));
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3414));
        env_chirho.bind_chirho(
            "local".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3413), TyVarChirho(3414)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(r_chirho.clone(), r_chirho),
                        TyChirho::io_chirho(a_chirho.clone()),
                    ],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // runExceptT :: IO a -> IO (Either e a)  (simplified)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3420));
        env_chirho.bind_chirho(
            "runExceptT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3420)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // throwE :: e -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3421));
        let e_chirho = TyChirho::VarChirho(TyVarChirho(3422));
        env_chirho.bind_chirho(
            "throwE".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3421), TyVarChirho(3422)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(e_chirho, TyChirho::io_chirho(a_chirho)),
            },
        );
    }

    // catchE :: IO a -> (e -> IO a) -> IO a
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3423));
        let e_chirho = TyChirho::VarChirho(TyVarChirho(3424));
        env_chirho.bind_chirho(
            "catchE".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3423), TyVarChirho(3424)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::io_chirho(a_chirho.clone()),
                        TyChirho::fun_chirho(e_chirho, TyChirho::io_chirho(a_chirho.clone())),
                    ],
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // runWriterT :: IO a -> IO (a, [w])
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3430));
        env_chirho.bind_chirho(
            "runWriterT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3430)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // tell :: w -> IO ()
    {
        let w_chirho = TyChirho::VarChirho(TyVarChirho(3431));
        env_chirho.bind_chirho(
            "tell".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3431)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    w_chirho,
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // runMaybeT :: IO a -> IO a  (simplified; wraps in Maybe at runtime)
    {
        let a_chirho = TyChirho::VarChirho(TyVarChirho(3440));
        env_chirho.bind_chirho(
            "runMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3440)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::io_chirho(a_chirho.clone()),
                    TyChirho::io_chirho(a_chirho),
                ),
            },
        );
    }

    // Data.Sequence.(|>) :: Seq a -> a -> Seq a
    {
        let seq_elem_chirho = TyVarChirho(3441);
        let seq_elem_ty_chirho = TyChirho::VarChirho(seq_elem_chirho);
        let seq_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Seq".to_string())),
            Box::new(seq_elem_ty_chirho.clone()),
        );
        let seq_snoc_scheme_chirho = SchemeChirho {
            vars_chirho: vec![seq_elem_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![seq_ty_chirho.clone(), seq_elem_ty_chirho],
                seq_ty_chirho,
            ),
        };
        for name_chirho in ["|>", "Data.Sequence.|>"] {
            env_chirho.bind_chirho(name_chirho.to_string(), seq_snoc_scheme_chirho.clone());
        }
    }

    // ── Data.Map operations ──
    // mapEmpty :: forall k v. Map k v
    {
        let map_empty_k_chirho = TyVarChirho(3208);
        let map_empty_v_chirho = TyVarChirho(3209);
        let map_empty_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(map_empty_k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(map_empty_v_chirho)),
        );
        let map_empty_scheme_chirho = SchemeChirho {
            vars_chirho: vec![map_empty_k_chirho, map_empty_v_chirho],
            preds_chirho: vec![],
            ty_chirho: map_empty_ty_chirho,
        };
        env_chirho.bind_chirho("mapEmpty".to_string(), map_empty_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.empty".to_string(), map_empty_scheme_chirho);
    }

    // mapSingleton :: k -> v -> Map k v
    {
        let map_singleton_k_var_chirho = TyVarChirho(3210);
        let map_singleton_v_var_chirho = TyVarChirho(3211);
        let k_chirho = TyChirho::VarChirho(map_singleton_k_var_chirho);
        let v_chirho = TyChirho::VarChirho(map_singleton_v_var_chirho);
        let map_singleton_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(k_chirho.clone()),
            )),
            Box::new(v_chirho.clone()),
        );
        env_chirho.bind_chirho(
            "mapSingleton".to_string(),
            SchemeChirho {
                vars_chirho: vec![map_singleton_k_var_chirho, map_singleton_v_var_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    k_chirho,
                    TyChirho::fun_chirho(v_chirho, map_singleton_ty_chirho),
                ),
            },
        );
    }

    // mapInsert :: Ord k => k -> v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3212);
        let v_chirho = TyVarChirho(3800);
        let map_insert_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let map_insert_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Ord".to_string(),
                ty_chirho: TyChirho::VarChirho(k_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::VarChirho(v_chirho),
                    map_insert_ty_chirho.clone(),
                ],
                map_insert_ty_chirho,
            ),
        };
        env_chirho.bind_chirho("mapInsert".to_string(), map_insert_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.insert".to_string(), map_insert_scheme_chirho);
    }

    // mapLookup :: Ord k => k -> Map k v -> Maybe v
    {
        let k_chirho = TyVarChirho(3801);
        let v_chirho = TyVarChirho(3213);
        let map_lookup_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let map_lookup_result_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        env_chirho.bind_chirho(
            "mapLookup".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::fun_chirho(
                        map_lookup_ty_chirho.clone(),
                        map_lookup_result_ty_chirho.clone(),
                    ),
                ),
            },
        );
        env_chirho.bind_chirho(
            "Data.Map.lookup".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::fun_chirho(
                        map_lookup_ty_chirho.clone(),
                        map_lookup_result_ty_chirho.clone(),
                    ),
                ),
            },
        );
        env_chirho.bind_chirho(
            "Data.Map.!".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    map_lookup_ty_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(k_chirho),
                        TyChirho::VarChirho(v_chirho),
                    ),
                ),
            },
        );
    }

    // mapSize :: Map k v -> Int
    {
        let map_size_k_chirho = TyVarChirho(38020);
        let map_size_v_chirho = TyVarChirho(38021);
        let map_size_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(map_size_k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(map_size_v_chirho)),
        );
        let map_size_scheme_chirho = SchemeChirho {
            vars_chirho: vec![map_size_k_chirho, map_size_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(map_size_ty_chirho, TyChirho::int_chirho()),
        };
        env_chirho.bind_chirho("mapSize".to_string(), map_size_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.size".to_string(), map_size_scheme_chirho);
    }

    // mapMember :: Ord k => k -> Map k v -> Bool
    {
        let k_chirho = TyVarChirho(3802);
        let v_chirho = TyVarChirho(38022);
        let map_member_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        env_chirho.bind_chirho(
            "mapMember".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::fun_chirho(map_member_ty_chirho.clone(), TyChirho::bool_chirho()),
                ),
            },
        );
        env_chirho.bind_chirho(
            "Data.Map.member".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::fun_chirho(map_member_ty_chirho, TyChirho::bool_chirho()),
                ),
            },
        );
    }

    // mapFromList :: Ord k => [(k, v)] -> Map k v
    {
        let k_chirho = TyVarChirho(3803);
        let v_chirho = TyVarChirho(3214);
        let map_from_list_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        env_chirho.bind_chirho(
            "mapFromList".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(k_chirho),
                        TyChirho::VarChirho(v_chirho),
                    ]))),
                    map_from_list_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "Data.Map.fromList".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(k_chirho),
                        TyChirho::VarChirho(v_chirho),
                    ]))),
                    map_from_list_ty_chirho,
                ),
            },
        );
    }

    // mapDelete :: Ord k => k -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3804);
        let v_chirho = TyVarChirho(38023);
        let map_delete_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let map_delete_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Ord".to_string(),
                ty_chirho: TyChirho::VarChirho(k_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(k_chirho), map_delete_ty_chirho.clone()],
                map_delete_ty_chirho,
            ),
        };
        env_chirho.bind_chirho("mapDelete".to_string(), map_delete_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.delete".to_string(), map_delete_scheme_chirho);
    }

    // mapToList :: Map k v -> [(k, v)]
    {
        let k_chirho = TyVarChirho(38024);
        let v_chirho = TyVarChirho(3220);
        let map_to_list_map_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let map_to_list_result_ty_chirho =
            TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                TyChirho::VarChirho(k_chirho),
                TyChirho::VarChirho(v_chirho),
            ])));
        let map_to_list_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                map_to_list_map_ty_chirho,
                map_to_list_result_ty_chirho,
            ),
        };
        env_chirho.bind_chirho("mapToList".to_string(), map_to_list_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.Map.assocs".to_string(),
            map_to_list_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho("Data.Map.toList".to_string(), map_to_list_scheme_chirho);
    }

    // mapKeys :: Map k v -> [k]
    {
        let k_chirho = TyVarChirho(38025);
        let v_chirho = TyVarChirho(38026);
        let map_keys_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Map".to_string())),
                        Box::new(TyChirho::VarChirho(k_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(v_chirho)),
                ),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(k_chirho))),
            ),
        };
        env_chirho.bind_chirho("mapKeys".to_string(), map_keys_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.keys".to_string(), map_keys_scheme_chirho);
    }

    // mapElems :: Map k v -> [v]
    {
        let k_chirho = TyVarChirho(38027);
        let v_chirho = TyVarChirho(3222);
        let map_elems_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Map".to_string())),
                        Box::new(TyChirho::VarChirho(k_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(v_chirho)),
                ),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(v_chirho))),
            ),
        };
        env_chirho.bind_chirho("mapElems".to_string(), map_elems_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.elems".to_string(), map_elems_scheme_chirho);
    }

    // mapNull :: Map k v -> Bool
    {
        let k_chirho = TyVarChirho(38028);
        let v_chirho = TyVarChirho(38029);
        let map_null_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Map".to_string())),
                        Box::new(TyChirho::VarChirho(k_chirho)),
                    )),
                    Box::new(TyChirho::VarChirho(v_chirho)),
                ),
                TyChirho::bool_chirho(),
            ),
        };
        env_chirho.bind_chirho("mapNull".to_string(), map_null_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.null".to_string(), map_null_scheme_chirho);
    }

    // mapMap :: (v -> w) -> Map k v -> Map k w
    {
        let k_chirho = TyVarChirho(38030);
        let v_chirho = TyVarChirho(3224);
        let w_chirho = TyVarChirho(3225);
        let map_map_input_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let map_map_result_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(w_chirho)),
        );
        let map_map_scheme_chirho = SchemeChirho {
            vars_chirho: vec![k_chirho, v_chirho, w_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(v_chirho),
                        TyChirho::VarChirho(w_chirho),
                    ),
                    map_map_input_ty_chirho,
                ],
                map_map_result_ty_chirho,
            ),
        };
        env_chirho.bind_chirho("mapMap".to_string(), map_map_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.Map.map".to_string(), map_map_scheme_chirho);
    }

    // Data.Map.mapKeys :: Ord k2 => (k1 -> k2) -> Map k1 v -> Map k2 v
    {
        let source_k_chirho = TyVarChirho(38031);
        let target_k_chirho = TyVarChirho(38032);
        let v_chirho = TyVarChirho(38033);
        let source_map_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(source_k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let target_map_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(target_k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        env_chirho.bind_chirho(
            "Data.Map.mapKeys".to_string(),
            SchemeChirho {
                vars_chirho: vec![source_k_chirho, target_k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(target_k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(source_k_chirho),
                            TyChirho::VarChirho(target_k_chirho),
                        ),
                        source_map_ty_chirho,
                    ],
                    target_map_ty_chirho,
                ),
            },
        );
    }
}
