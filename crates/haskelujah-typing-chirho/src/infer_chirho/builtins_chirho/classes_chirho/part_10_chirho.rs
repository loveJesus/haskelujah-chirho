// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 10 of 14: original lines 16318-17269 of
//! `seed_builtins_chirho`, in order (27 named binds).

use super::super::transformer_types_chirho::{
    mk_maybe_t_ty_chirho, mk_reader_t_ty_chirho, mk_state_t_ty_chirho, mk_state_ty_chirho,
};
use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_10_chirho(env_chirho: &mut TyEnvChirho) {
    // traverse :: (a -> f b) -> t a -> f (t b)  (simplified, Applicative f not enforced)
    {
        let trav_t_chirho = TyVarChirho(9083);
        let trav_a_chirho = TyVarChirho(9084);
        let trav_b_chirho = TyVarChirho(9085);
        let trav_f_chirho = TyVarChirho(9086);
        let traverse_scheme_chirho = SchemeChirho {
            vars_chirho: vec![trav_t_chirho, trav_a_chirho, trav_b_chirho, trav_f_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::FunChirho(
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(trav_a_chirho)),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_f_chirho)),
                        Box::new(TyChirho::VarChirho(trav_b_chirho)),
                    )),
                    MultChirho::ManyChirho,
                )),
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_t_chirho)),
                        Box::new(TyChirho::VarChirho(trav_a_chirho)),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_f_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_t_chirho)),
                            Box::new(TyChirho::VarChirho(trav_b_chirho)),
                        )),
                    )),
                    MultChirho::ManyChirho,
                )),
                MultChirho::ManyChirho,
            ),
        };
        env_chirho.bind_chirho("traverse".to_string(), traverse_scheme_chirho.clone());
        env_chirho.bind_chirho("mapM".to_string(), traverse_scheme_chirho);
        env_chirho.bind_chirho(
            "sequenceA".to_string(),
            SchemeChirho {
                vars_chirho: vec![trav_t_chirho, trav_a_chirho, trav_f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_t_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_f_chirho)),
                            Box::new(TyChirho::VarChirho(trav_a_chirho)),
                        )),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_f_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_t_chirho)),
                            Box::new(TyChirho::VarChirho(trav_a_chirho)),
                        )),
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
        env_chirho.bind_chirho(
            "sequence".to_string(),
            SchemeChirho {
                vars_chirho: vec![trav_t_chirho, trav_a_chirho, trav_f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_t_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_f_chirho)),
                            Box::new(TyChirho::VarChirho(trav_a_chirho)),
                        )),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_f_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav_t_chirho)),
                            Box::new(TyChirho::VarChirho(trav_a_chirho)),
                        )),
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );

        let trav1_t_chirho = TyVarChirho(9087);
        let trav1_a_chirho = TyVarChirho(9088);
        let trav1_b_chirho = TyVarChirho(9089);
        let trav1_f_chirho = TyVarChirho(9090);
        let traverse1_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                trav1_t_chirho,
                trav1_a_chirho,
                trav1_b_chirho,
                trav1_f_chirho,
            ],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Traversable1".to_string(),
                    ty_chirho: TyChirho::VarChirho(trav1_t_chirho),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "Apply".to_string(),
                    ty_chirho: TyChirho::VarChirho(trav1_f_chirho),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::FunChirho(
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(trav1_a_chirho)),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav1_f_chirho)),
                        Box::new(TyChirho::VarChirho(trav1_b_chirho)),
                    )),
                    MultChirho::ManyChirho,
                )),
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav1_t_chirho)),
                        Box::new(TyChirho::VarChirho(trav1_a_chirho)),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav1_f_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav1_t_chirho)),
                            Box::new(TyChirho::VarChirho(trav1_b_chirho)),
                        )),
                    )),
                    MultChirho::ManyChirho,
                )),
                MultChirho::ManyChirho,
            ),
        };
        env_chirho.bind_chirho("traverse1".to_string(), traverse1_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "sequence1".to_string(),
            SchemeChirho {
                vars_chirho: vec![trav1_t_chirho, trav1_a_chirho, trav1_f_chirho],
                preds_chirho: vec![
                    SchemePredChirho {
                        class_name_chirho: "Traversable1".to_string(),
                        ty_chirho: TyChirho::VarChirho(trav1_t_chirho),
                        extra_tys_chirho: vec![],
                    },
                    SchemePredChirho {
                        class_name_chirho: "Apply".to_string(),
                        ty_chirho: TyChirho::VarChirho(trav1_f_chirho),
                        extra_tys_chirho: vec![],
                    },
                ],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav1_t_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav1_f_chirho)),
                            Box::new(TyChirho::VarChirho(trav1_a_chirho)),
                        )),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav1_f_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(trav1_t_chirho)),
                            Box::new(TyChirho::VarChirho(trav1_a_chirho)),
                        )),
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // indexed-traversable core methods
    {
        let index_i_chirho = TyVarChirho(9091);
        let indexed_f_chirho = TyVarChirho(9092);
        let indexed_t_chirho = TyVarChirho(9093);
        let indexed_g_chirho = TyVarChirho(9094);
        let indexed_a_chirho = TyVarChirho(9095);
        let indexed_b_chirho = TyVarChirho(9096);
        let indexed_m_chirho = TyVarChirho(9097);

        let foldable_with_index_pred_chirho = SchemePredChirho {
            class_name_chirho: "FoldableWithIndex".to_string(),
            ty_chirho: TyChirho::VarChirho(index_i_chirho),
            extra_tys_chirho: vec![TyChirho::VarChirho(indexed_f_chirho)],
        };
        let functor_with_index_pred_chirho = SchemePredChirho {
            class_name_chirho: "FunctorWithIndex".to_string(),
            ty_chirho: TyChirho::VarChirho(index_i_chirho),
            extra_tys_chirho: vec![TyChirho::VarChirho(indexed_f_chirho)],
        };
        let traversable_with_index_pred_chirho = SchemePredChirho {
            class_name_chirho: "TraversableWithIndex".to_string(),
            ty_chirho: TyChirho::VarChirho(index_i_chirho),
            extra_tys_chirho: vec![TyChirho::VarChirho(indexed_t_chirho)],
        };
        let monoid_pred_chirho = SchemePredChirho {
            class_name_chirho: "Monoid".to_string(),
            ty_chirho: TyChirho::VarChirho(indexed_m_chirho),
            extra_tys_chirho: vec![],
        };
        let applicative_pred_chirho = SchemePredChirho {
            class_name_chirho: "Applicative".to_string(),
            ty_chirho: TyChirho::VarChirho(indexed_g_chirho),
            extra_tys_chirho: vec![],
        };

        let fa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(indexed_f_chirho)),
            Box::new(TyChirho::VarChirho(indexed_a_chirho)),
        );
        let fb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(indexed_f_chirho)),
            Box::new(TyChirho::VarChirho(indexed_b_chirho)),
        );
        let ta_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(indexed_t_chirho)),
            Box::new(TyChirho::VarChirho(indexed_a_chirho)),
        );
        let tb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(indexed_t_chirho)),
            Box::new(TyChirho::VarChirho(indexed_b_chirho)),
        );
        let gb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(indexed_g_chirho)),
            Box::new(TyChirho::VarChirho(indexed_b_chirho)),
        );
        let gtb_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(indexed_g_chirho)),
            Box::new(tb_chirho.clone()),
        );

        let indexed_modules_chirho = [
            "WithIndex",
            "Data.Functor.WithIndex",
            "Data.Foldable.WithIndex",
            "Data.Traversable.WithIndex",
        ];
        let mut bind_indexed_method_chirho = |name_chirho: &str, scheme_chirho: SchemeChirho| {
            env_chirho.bind_chirho(name_chirho.to_string(), scheme_chirho.clone());
            for module_name_chirho in indexed_modules_chirho {
                env_chirho.bind_chirho(
                    format!("{module_name_chirho}.{name_chirho}"),
                    scheme_chirho.clone(),
                );
            }
        };

        let imap_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_f_chirho,
                indexed_a_chirho,
                indexed_b_chirho,
            ],
            preds_chirho: vec![functor_with_index_pred_chirho],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                        ],
                        TyChirho::VarChirho(indexed_b_chirho),
                    ),
                    fa_chirho.clone(),
                ],
                fb_chirho,
            ),
        };
        bind_indexed_method_chirho("imap", imap_scheme_chirho);

        let imap_default_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_t_chirho,
                indexed_a_chirho,
                indexed_b_chirho,
            ],
            preds_chirho: vec![traversable_with_index_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                        ],
                        TyChirho::VarChirho(indexed_b_chirho),
                    ),
                    ta_chirho.clone(),
                ],
                tb_chirho.clone(),
            ),
        };
        bind_indexed_method_chirho("imapDefault", imap_default_scheme_chirho);

        let ifold_map_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_f_chirho,
                indexed_a_chirho,
                indexed_m_chirho,
            ],
            preds_chirho: vec![
                foldable_with_index_pred_chirho.clone(),
                monoid_pred_chirho.clone(),
            ],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                        ],
                        TyChirho::VarChirho(indexed_m_chirho),
                    ),
                    fa_chirho.clone(),
                ],
                TyChirho::VarChirho(indexed_m_chirho),
            ),
        };
        bind_indexed_method_chirho("ifoldMap", ifold_map_scheme_chirho);

        let ifold_map_default_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_t_chirho,
                indexed_a_chirho,
                indexed_m_chirho,
            ],
            preds_chirho: vec![
                traversable_with_index_pred_chirho.clone(),
                monoid_pred_chirho,
            ],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                        ],
                        TyChirho::VarChirho(indexed_m_chirho),
                    ),
                    ta_chirho.clone(),
                ],
                TyChirho::VarChirho(indexed_m_chirho),
            ),
        };
        bind_indexed_method_chirho("ifoldMapDefault", ifold_map_default_scheme_chirho);

        let ifoldr_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_f_chirho,
                indexed_a_chirho,
                indexed_b_chirho,
            ],
            preds_chirho: vec![foldable_with_index_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                            TyChirho::VarChirho(indexed_b_chirho),
                        ],
                        TyChirho::VarChirho(indexed_b_chirho),
                    ),
                    TyChirho::VarChirho(indexed_b_chirho),
                    fa_chirho.clone(),
                ],
                TyChirho::VarChirho(indexed_b_chirho),
            ),
        };
        bind_indexed_method_chirho("ifoldr", ifoldr_scheme_chirho.clone());
        bind_indexed_method_chirho("ifoldr'", ifoldr_scheme_chirho);

        let ifoldl_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_f_chirho,
                indexed_a_chirho,
                indexed_b_chirho,
            ],
            preds_chirho: vec![foldable_with_index_pred_chirho],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_b_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                        ],
                        TyChirho::VarChirho(indexed_b_chirho),
                    ),
                    TyChirho::VarChirho(indexed_b_chirho),
                    fa_chirho,
                ],
                TyChirho::VarChirho(indexed_b_chirho),
            ),
        };
        bind_indexed_method_chirho("ifoldl", ifoldl_scheme_chirho.clone());
        bind_indexed_method_chirho("ifoldl'", ifoldl_scheme_chirho);

        let itraverse_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                index_i_chirho,
                indexed_t_chirho,
                indexed_g_chirho,
                indexed_a_chirho,
                indexed_b_chirho,
            ],
            preds_chirho: vec![traversable_with_index_pred_chirho, applicative_pred_chirho],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(index_i_chirho),
                            TyChirho::VarChirho(indexed_a_chirho),
                        ],
                        gb_chirho,
                    ),
                    ta_chirho,
                ],
                gtb_chirho,
            ),
        };
        bind_indexed_method_chirho("itraverse", itraverse_scheme_chirho);
    }

    // ── Monad transformer infrastructure ──
    //
    // MaybeT is a newtype: newtype MaybeT m a = MaybeT { runMaybeT :: m (Maybe a) }
    // StateT is a newtype: newtype StateT s m a = StateT { runStateT :: s -> m (a, s) }
    //
    // We seed constructor and accessor types with their real transformer shape
    // so partially-applied transformer heads stay well-kinded and unify
    // correctly in package code.

    // MaybeT :: m (Maybe a) -> MaybeT m a
    // runMaybeT :: MaybeT m a -> m (Maybe a)
    {
        let mt_m_chirho = TyVarChirho(3500);
        let mt_a_chirho = TyVarChirho(3501);
        let maybe_a_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::VarChirho(mt_a_chirho)),
        );
        let monadic_maybe_a_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(mt_m_chirho)),
            Box::new(maybe_a_ty_chirho.clone()),
        );
        let maybet_ty_chirho = mk_maybe_t_ty_chirho(
            TyChirho::VarChirho(mt_m_chirho),
            TyChirho::VarChirho(mt_a_chirho),
        );
        env_chirho.bind_chirho(
            "MaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![mt_m_chirho, mt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    monadic_maybe_a_ty_chirho.clone(),
                    maybet_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "runMaybeT".to_string(),
            SchemeChirho {
                vars_chirho: vec![mt_m_chirho, mt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(maybet_ty_chirho, monadic_maybe_a_ty_chirho),
            },
        );
    }

    // StateT :: (s -> m (a, s)) -> StateT s m a
    // runStateT :: StateT s m a -> s -> m (a, s)
    {
        let st_s_chirho = TyVarChirho(3501);
        let st_m_chirho = TyVarChirho(3502);
        let st_a_chirho = TyVarChirho(3503);
        let st_fn_ty_chirho = TyChirho::fun_chirho(
            TyChirho::VarChirho(st_s_chirho),
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(st_m_chirho)),
                Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(st_a_chirho),
                    TyChirho::VarChirho(st_s_chirho),
                ])),
            ),
        );
        let statet_ty_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(st_s_chirho),
            TyChirho::VarChirho(st_m_chirho),
            TyChirho::VarChirho(st_a_chirho),
        );
        env_chirho.bind_chirho(
            "StateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![st_s_chirho, st_m_chirho, st_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(st_fn_ty_chirho.clone(), statet_ty_chirho.clone()),
            },
        );
        env_chirho.bind_chirho(
            "runStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![st_s_chirho, st_m_chirho, st_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(statet_ty_chirho, st_fn_ty_chirho),
            },
        );
    }

    // lift :: forall t m a. m a -> t m a
    // Simplified: polymorphic binding for MonadTrans-like usage
    {
        let lt_m_chirho = TyVarChirho(3503);
        let lt_t_chirho = TyVarChirho(3504);
        let lt_a_chirho = TyVarChirho(3505);
        let lt_ma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(lt_m_chirho)),
            Box::new(TyChirho::VarChirho(lt_a_chirho)),
        );
        let lt_tma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(lt_t_chirho)),
                Box::new(TyChirho::VarChirho(lt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(lt_a_chirho)),
        );
        env_chirho.bind_chirho(
            "lift".to_string(),
            SchemeChirho {
                vars_chirho: vec![lt_m_chirho, lt_t_chirho, lt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(lt_ma_chirho, lt_tma_chirho),
            },
        );
    }

    // -----------------------------------------------------------------------
    // StateT monad operations
    // -----------------------------------------------------------------------

    // get :: StateT s m s
    {
        let gs_s_chirho = TyVarChirho(3510);
        let gs_m_chirho = TyVarChirho(3511);
        let state_t_s_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(gs_s_chirho),
            TyChirho::VarChirho(gs_m_chirho),
            TyChirho::VarChirho(gs_s_chirho),
        );
        env_chirho.bind_chirho(
            "get".to_string(),
            SchemeChirho {
                vars_chirho: vec![gs_s_chirho, gs_m_chirho],
                preds_chirho: vec![],
                ty_chirho: state_t_s_chirho,
            },
        );
    }

    // put :: s -> StateT s m ()
    {
        let ps_s_chirho = TyVarChirho(3512);
        let ps_m_chirho = TyVarChirho(3513);
        let state_t_unit_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(ps_s_chirho),
            TyChirho::VarChirho(ps_m_chirho),
            TyChirho::TupleChirho(vec![]),
        );
        env_chirho.bind_chirho(
            "put".to_string(),
            SchemeChirho {
                vars_chirho: vec![ps_s_chirho, ps_m_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(ps_s_chirho),
                    state_t_unit_chirho,
                ),
            },
        );
    }

    // modify :: (s -> s) -> StateT s m ()
    {
        let ms_s_chirho = TyVarChirho(3514);
        let ms_m_chirho = TyVarChirho(3515);
        let state_t_unit_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(ms_s_chirho),
            TyChirho::VarChirho(ms_m_chirho),
            TyChirho::TupleChirho(vec![]),
        );
        env_chirho.bind_chirho(
            "modify".to_string(),
            SchemeChirho {
                vars_chirho: vec![ms_s_chirho, ms_m_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(ms_s_chirho),
                        TyChirho::VarChirho(ms_s_chirho),
                    ),
                    state_t_unit_chirho,
                ),
            },
        );
    }

    // state :: (s -> (a, s)) -> StateT s m a
    {
        let ss_s_chirho = TyVarChirho(3516);
        let ss_m_chirho = TyVarChirho(3517);
        let ss_a_chirho = TyVarChirho(3518);
        let state_t_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(ss_s_chirho),
            TyChirho::VarChirho(ss_m_chirho),
            TyChirho::VarChirho(ss_a_chirho),
        );
        env_chirho.bind_chirho(
            "state".to_string(),
            SchemeChirho {
                vars_chirho: vec![ss_s_chirho, ss_m_chirho, ss_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(ss_s_chirho),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(ss_a_chirho),
                            TyChirho::VarChirho(ss_s_chirho),
                        ]),
                    ),
                    state_t_chirho,
                ),
            },
        );
    }

    // evalState :: State s a -> s -> a
    {
        let es_s_chirho = TyVarChirho(3519);
        let es_a_chirho = TyVarChirho(3521);
        let state_ty_chirho = mk_state_ty_chirho(
            TyChirho::VarChirho(es_s_chirho),
            TyChirho::VarChirho(es_a_chirho),
        );
        env_chirho.bind_chirho(
            "evalState".to_string(),
            SchemeChirho {
                vars_chirho: vec![es_s_chirho, es_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_ty_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(es_s_chirho),
                        TyChirho::VarChirho(es_a_chirho),
                    ),
                ),
            },
        );
    }

    // execState :: State s a -> s -> s
    {
        let xs_s_chirho = TyVarChirho(3522);
        let xs_a_chirho = TyVarChirho(3524);
        let state_ty_chirho = mk_state_ty_chirho(
            TyChirho::VarChirho(xs_s_chirho),
            TyChirho::VarChirho(xs_a_chirho),
        );
        env_chirho.bind_chirho(
            "execState".to_string(),
            SchemeChirho {
                vars_chirho: vec![xs_s_chirho, xs_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_ty_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(xs_s_chirho),
                        TyChirho::VarChirho(xs_s_chirho),
                    ),
                ),
            },
        );
    }

    // bindStateT :: StateT s m a -> (a -> StateT s m b) -> StateT s m b
    {
        let bs_s_chirho = TyVarChirho(3525);
        let bs_m_chirho = TyVarChirho(3526);
        let bs_a_chirho = TyVarChirho(3527);
        let bs_b_chirho = TyVarChirho(3528);
        let state_t_a_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(bs_s_chirho),
            TyChirho::VarChirho(bs_m_chirho),
            TyChirho::VarChirho(bs_a_chirho),
        );
        let state_t_b_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(bs_s_chirho),
            TyChirho::VarChirho(bs_m_chirho),
            TyChirho::VarChirho(bs_b_chirho),
        );
        env_chirho.bind_chirho(
            "bindStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![bs_s_chirho, bs_m_chirho, bs_a_chirho, bs_b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_t_a_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(bs_a_chirho),
                            state_t_b_chirho.clone(),
                        ),
                        state_t_b_chirho,
                    ),
                ),
            },
        );
    }

    // returnStateT :: a -> StateT s m a
    {
        let rs_s_chirho = TyVarChirho(3529);
        let rs_m_chirho = TyVarChirho(3530);
        let rs_a_chirho = TyVarChirho(3531);
        let state_t_chirho = mk_state_t_ty_chirho(
            TyChirho::VarChirho(rs_s_chirho),
            TyChirho::VarChirho(rs_m_chirho),
            TyChirho::VarChirho(rs_a_chirho),
        );
        env_chirho.bind_chirho(
            "returnStateT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rs_s_chirho, rs_m_chirho, rs_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(rs_a_chirho), state_t_chirho),
            },
        );
    }

    // runState :: State s a -> s -> (a, s)
    {
        let rns_s_chirho = TyVarChirho(3532);
        let rns_a_chirho = TyVarChirho(3534);
        let state_ty_chirho = mk_state_ty_chirho(
            TyChirho::VarChirho(rns_s_chirho),
            TyChirho::VarChirho(rns_a_chirho),
        );
        env_chirho.bind_chirho(
            "runState".to_string(),
            SchemeChirho {
                vars_chirho: vec![rns_s_chirho, rns_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    state_ty_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(rns_s_chirho),
                        TyChirho::TupleChirho(vec![
                            TyChirho::VarChirho(rns_a_chirho),
                            TyChirho::VarChirho(rns_s_chirho),
                        ]),
                    ),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // ReaderT monad transformer
    // -----------------------------------------------------------------------

    // ReaderT :: (r -> m a) -> ReaderT r m a
    // runReaderT :: ReaderT r m a -> r -> m a
    // The runtime shim remains first-order, but the public surface keeps the
    // GHC-shaped inner monad so partially-applied ReaderT code typechecks.
    {
        let rt_r_chirho = TyVarChirho(3530);
        let rt_m_chirho = TyVarChirho(3531);
        let rt_a_chirho = TyVarChirho(3532);
        let rt_inner_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(rt_m_chirho)),
            Box::new(TyChirho::VarChirho(rt_a_chirho)),
        );
        let rt_fn_ty_chirho =
            TyChirho::fun_chirho(TyChirho::VarChirho(rt_r_chirho), rt_inner_ty_chirho);
        let readert_ty_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(rt_r_chirho),
            TyChirho::VarChirho(rt_m_chirho),
            TyChirho::VarChirho(rt_a_chirho),
        );
        env_chirho.bind_chirho(
            "ReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rt_r_chirho, rt_m_chirho, rt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(rt_fn_ty_chirho.clone(), readert_ty_chirho.clone()),
            },
        );
        env_chirho.bind_chirho(
            "runReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rt_r_chirho, rt_m_chirho, rt_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(readert_ty_chirho.clone(), rt_fn_ty_chirho.clone()),
            },
        );
    }

    // ask :: Monad m => ReaderT r m r
    {
        let ar_r_chirho = TyVarChirho(3533);
        let ar_m_chirho = TyVarChirho(3534);
        let ask_ty_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(ar_r_chirho),
            TyChirho::VarChirho(ar_m_chirho),
            TyChirho::VarChirho(ar_r_chirho),
        );
        env_chirho.bind_chirho(
            "ask".to_string(),
            SchemeChirho {
                vars_chirho: vec![ar_r_chirho, ar_m_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(ar_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: ask_ty_chirho,
            },
        );
    }

    // local :: (r -> r) -> ReaderT r m a -> ReaderT r m a
    {
        let lr_r_chirho = TyVarChirho(3535);
        let lr_m_chirho = TyVarChirho(3536);
        let la_chirho = TyVarChirho(3537);
        let readert_ra_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(lr_r_chirho),
            TyChirho::VarChirho(lr_m_chirho),
            TyChirho::VarChirho(la_chirho),
        );
        env_chirho.bind_chirho(
            "local".to_string(),
            SchemeChirho {
                vars_chirho: vec![lr_r_chirho, lr_m_chirho, la_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(lr_r_chirho),
                        TyChirho::VarChirho(lr_r_chirho),
                    ),
                    TyChirho::fun_chirho(readert_ra_chirho.clone(), readert_ra_chirho),
                ),
            },
        );
    }

    // bindReaderT :: Monad m => ReaderT r m a -> (a -> ReaderT r m b) -> ReaderT r m b
    {
        let br_r_chirho = TyVarChirho(3538);
        let br_m_chirho = TyVarChirho(3539);
        let ba_chirho = TyVarChirho(3540);
        let bb_chirho = TyVarChirho(3541);
        let readert_ra_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(br_r_chirho),
            TyChirho::VarChirho(br_m_chirho),
            TyChirho::VarChirho(ba_chirho),
        );
        let readert_rb_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(br_r_chirho),
            TyChirho::VarChirho(br_m_chirho),
            TyChirho::VarChirho(bb_chirho),
        );
        env_chirho.bind_chirho(
            "bindReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![br_r_chirho, br_m_chirho, ba_chirho, bb_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(br_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    readert_ra_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(ba_chirho),
                            readert_rb_chirho.clone(),
                        ),
                        readert_rb_chirho,
                    ),
                ),
            },
        );
    }

    // returnReaderT :: Applicative m => a -> ReaderT r m a
    {
        let rr_r_chirho = TyVarChirho(3542);
        let rr_m_chirho = TyVarChirho(3543);
        let ra_chirho = TyVarChirho(3544);
        let readert_ty_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(rr_r_chirho),
            TyChirho::VarChirho(rr_m_chirho),
            TyChirho::VarChirho(ra_chirho),
        );
        env_chirho.bind_chirho(
            "returnReaderT".to_string(),
            SchemeChirho {
                vars_chirho: vec![rr_r_chirho, rr_m_chirho, ra_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Applicative".to_string(),
                    ty_chirho: TyChirho::VarChirho(rr_m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(TyChirho::VarChirho(ra_chirho), readert_ty_chirho),
            },
        );
    }

    // runReader :: ReaderT r m a -> r -> a  (alias surface)
    {
        let rnr_r_chirho = TyVarChirho(3545);
        let rnr_m_chirho = TyVarChirho(3546);
        let rnr_a_chirho = TyVarChirho(3547);
        let readert_ty_chirho = mk_reader_t_ty_chirho(
            TyChirho::VarChirho(rnr_r_chirho),
            TyChirho::VarChirho(rnr_m_chirho),
            TyChirho::VarChirho(rnr_a_chirho),
        );
        env_chirho.bind_chirho(
            "runReader".to_string(),
            SchemeChirho {
                vars_chirho: vec![rnr_r_chirho, rnr_m_chirho, rnr_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    readert_ty_chirho,
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(rnr_r_chirho),
                        TyChirho::VarChirho(rnr_a_chirho),
                    ),
                ),
            },
        );
    }
}
