// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 08 of 14: original lines 14433-15382 of
//! `seed_builtins_chirho`, in order (49 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_08_chirho(env_chirho: &mut TyEnvChirho) {
    // seq :: forall a b. a -> b -> b
    let seq_a_chirho = TyVarChirho(3402);
    let seq_b_chirho = TyVarChirho(3403);
    env_chirho.bind_chirho(
        "seq".to_string(),
        SchemeChirho {
            vars_chirho: vec![seq_a_chirho, seq_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(seq_a_chirho),
                    TyChirho::VarChirho(seq_b_chirho),
                ],
                TyChirho::VarChirho(seq_b_chirho),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // NFData / deepseq / force / evaluate
    // -----------------------------------------------------------------------

    // deepseq :: forall a b. NFData a => a -> b -> b
    let ds_a_chirho = TyVarChirho(3410);
    let ds_b_chirho = TyVarChirho(3411);
    env_chirho.bind_chirho(
        "deepseq".to_string(),
        SchemeChirho {
            vars_chirho: vec![ds_a_chirho, ds_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "NFData".to_string(),
                ty_chirho: TyChirho::VarChirho(ds_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(ds_a_chirho),
                    TyChirho::VarChirho(ds_b_chirho),
                ],
                TyChirho::VarChirho(ds_b_chirho),
            ),
        },
    );

    // force :: forall a. NFData a => a -> a
    let force_a_chirho = TyVarChirho(3412);
    env_chirho.bind_chirho(
        "force".to_string(),
        SchemeChirho {
            vars_chirho: vec![force_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "NFData".to_string(),
                ty_chirho: TyChirho::VarChirho(force_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(force_a_chirho)],
                TyChirho::VarChirho(force_a_chirho),
            ),
        },
    );

    // evaluate :: forall a. a -> IO a
    let eval_a_chirho = TyVarChirho(3413);
    env_chirho.bind_chirho(
        "evaluate".to_string(),
        SchemeChirho {
            vars_chirho: vec![eval_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(eval_a_chirho)],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("IO".to_string())),
                    Box::new(TyChirho::VarChirho(eval_a_chirho)),
                ),
            ),
        },
    );

    // rnf :: forall a. NFData a => a -> ()
    let rnf_a_chirho = TyVarChirho(3414);
    env_chirho.bind_chirho(
        "rnf".to_string(),
        SchemeChirho {
            vars_chirho: vec![rnf_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "NFData".to_string(),
                ty_chirho: TyChirho::VarChirho(rnf_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(rnf_a_chirho)],
                TyChirho::TupleChirho(vec![]),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // Semigroup / Monoid
    // -----------------------------------------------------------------------

    // (<>) :: forall a. Semigroup a => a -> a -> a
    let sg_v_chirho = TyVarChirho(3600);
    env_chirho.bind_chirho(
        "<>".to_string(),
        SchemeChirho {
            vars_chirho: vec![sg_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Semigroup".to_string(),
                ty_chirho: TyChirho::VarChirho(sg_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(sg_v_chirho),
                    TyChirho::VarChirho(sg_v_chirho),
                ],
                TyChirho::VarChirho(sg_v_chirho),
            ),
        },
    );

    // mempty :: forall a. Monoid a => a
    let mon_v_chirho = TyVarChirho(3601);
    env_chirho.bind_chirho(
        "mempty".to_string(),
        SchemeChirho {
            vars_chirho: vec![mon_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monoid".to_string(),
                ty_chirho: TyChirho::VarChirho(mon_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::VarChirho(mon_v_chirho),
        },
    );

    // mconcat :: forall a. Monoid a => [a] -> a
    let mc_v_chirho = TyVarChirho(3602);
    env_chirho.bind_chirho(
        "mconcat".to_string(),
        SchemeChirho {
            vars_chirho: vec![mc_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Monoid".to_string(),
                ty_chirho: TyChirho::VarChirho(mc_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mc_v_chirho))),
                TyChirho::VarChirho(mc_v_chirho),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // Higher-order *By list Prelude functions
    // -----------------------------------------------------------------------

    // sortBy :: forall a. (a -> a -> Ordering) -> [a] -> [a]
    {
        let sb_a_chirho = TyVarChirho(3700);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "sortBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![sb_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(sb_a_chirho),
                                TyChirho::VarChirho(sb_a_chirho),
                            ],
                            ordering_chirho.clone(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sb_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sb_a_chirho))),
                ),
            },
        );
    }

    // insertBy :: forall a. (a -> a -> Ordering) -> a -> [a] -> [a]
    {
        let ib_a_chirho = TyVarChirho(3701);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "insertBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![ib_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(ib_a_chirho),
                                TyChirho::VarChirho(ib_a_chirho),
                            ],
                            ordering_chirho,
                        ),
                        TyChirho::VarChirho(ib_a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ib_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ib_a_chirho))),
                ),
            },
        );
    }

    // nubBy :: forall a. (a -> a -> Bool) -> [a] -> [a]
    {
        let nb_a_chirho = TyVarChirho(3702);
        env_chirho.bind_chirho(
            "nubBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![nb_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(nb_a_chirho),
                                TyChirho::VarChirho(nb_a_chirho),
                            ],
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nb_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nb_a_chirho))),
                ),
            },
        );
    }

    // maximumBy :: forall a. (a -> a -> Ordering) -> [a] -> a
    {
        let mx_a_chirho = TyVarChirho(3703);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "maximumBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![mx_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(mx_a_chirho),
                                TyChirho::VarChirho(mx_a_chirho),
                            ],
                            ordering_chirho,
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mx_a_chirho))),
                    ],
                    TyChirho::VarChirho(mx_a_chirho),
                ),
            },
        );
    }

    // minimumBy :: forall a. (a -> a -> Ordering) -> [a] -> a
    {
        let mn_a_chirho = TyVarChirho(3704);
        let ordering_chirho = TyChirho::ConChirho("Ordering".to_string());
        env_chirho.bind_chirho(
            "minimumBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![mn_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(mn_a_chirho),
                                TyChirho::VarChirho(mn_a_chirho),
                            ],
                            ordering_chirho,
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mn_a_chirho))),
                    ],
                    TyChirho::VarChirho(mn_a_chirho),
                ),
            },
        );
    }

    // on :: forall a b c. (b -> b -> c) -> (a -> b) -> a -> a -> c
    {
        let on_a_chirho = TyVarChirho(3705);
        let on_b_chirho = TyVarChirho(3706);
        let on_c_chirho = TyVarChirho(3707);
        env_chirho.bind_chirho(
            "on".to_string(),
            SchemeChirho {
                vars_chirho: vec![on_a_chirho, on_b_chirho, on_c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(on_b_chirho),
                                TyChirho::VarChirho(on_b_chirho),
                            ],
                            TyChirho::VarChirho(on_c_chirho),
                        ),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(on_a_chirho),
                            TyChirho::VarChirho(on_b_chirho),
                        ),
                        TyChirho::VarChirho(on_a_chirho),
                        TyChirho::VarChirho(on_a_chirho),
                    ],
                    TyChirho::VarChirho(on_c_chirho),
                ),
            },
        );
    }

    // find :: forall a. (a -> Bool) -> [a] -> Maybe a
    {
        let find_a_chirho = TyVarChirho(3710);
        env_chirho.bind_chirho(
            "find".to_string(),
            SchemeChirho {
                vars_chirho: vec![find_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(find_a_chirho),
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(find_a_chirho))),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(find_a_chirho)),
                    ),
                ),
            },
        );
    }

    // groupBy :: forall a. (a -> a -> Bool) -> [a] -> [[a]]
    {
        let gb_a_chirho = TyVarChirho(3711);
        env_chirho.bind_chirho(
            "groupBy".to_string(),
            SchemeChirho {
                vars_chirho: vec![gb_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::VarChirho(gb_a_chirho),
                                TyChirho::VarChirho(gb_a_chirho),
                            ],
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(gb_a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                        TyChirho::VarChirho(gb_a_chirho),
                    )))),
                ),
            },
        );
    }

    // -----------------------------------------------------------------------
    // Higher-order list Prelude functions
    // -----------------------------------------------------------------------

    // map :: forall a b. (a -> b) -> [a] -> [b]
    let map_a_chirho = TyVarChirho(3000);
    let map_b_chirho = TyVarChirho(3001);
    env_chirho.bind_chirho(
        "map".to_string(),
        SchemeChirho {
            vars_chirho: vec![map_a_chirho, map_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(map_a_chirho),
                        TyChirho::VarChirho(map_b_chirho),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(map_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(map_b_chirho))),
            ),
        },
    );

    // filter :: forall a. (a -> Bool) -> [a] -> [a]
    let filter_a_chirho = TyVarChirho(3010);
    env_chirho.bind_chirho(
        "filter".to_string(),
        SchemeChirho {
            vars_chirho: vec![filter_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(filter_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(filter_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(filter_a_chirho))),
            ),
        },
    );

    // foldr :: forall a b. (a -> b -> b) -> b -> [a] -> b
    let foldr_a_chirho = TyVarChirho(3020);
    let foldr_b_chirho = TyVarChirho(3021);
    env_chirho.bind_chirho(
        "foldr".to_string(),
        SchemeChirho {
            vars_chirho: vec![foldr_a_chirho, foldr_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(foldr_a_chirho),
                            TyChirho::VarChirho(foldr_b_chirho),
                        ],
                        TyChirho::VarChirho(foldr_b_chirho),
                    ),
                    TyChirho::VarChirho(foldr_b_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(foldr_a_chirho))),
                ],
                TyChirho::VarChirho(foldr_b_chirho),
            ),
        },
    );

    // foldl :: forall a b. (b -> a -> b) -> b -> [a] -> b
    let foldl_a_chirho = TyVarChirho(3030);
    let foldl_b_chirho = TyVarChirho(3031);
    env_chirho.bind_chirho(
        "foldl".to_string(),
        SchemeChirho {
            vars_chirho: vec![foldl_a_chirho, foldl_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(foldl_b_chirho),
                            TyChirho::VarChirho(foldl_a_chirho),
                        ],
                        TyChirho::VarChirho(foldl_b_chirho),
                    ),
                    TyChirho::VarChirho(foldl_b_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(foldl_a_chirho))),
                ],
                TyChirho::VarChirho(foldl_b_chirho),
            ),
        },
    );

    // head :: forall a. [a] -> a
    let head_a_chirho = TyVarChirho(3040);
    env_chirho.bind_chirho(
        "head".to_string(),
        SchemeChirho {
            vars_chirho: vec![head_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(head_a_chirho))),
                TyChirho::VarChirho(head_a_chirho),
            ),
        },
    );

    // tail :: forall a. [a] -> [a]
    let tail_a_chirho = TyVarChirho(3050);
    env_chirho.bind_chirho(
        "tail".to_string(),
        SchemeChirho {
            vars_chirho: vec![tail_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tail_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tail_a_chirho))),
            ),
        },
    );

    // null :: forall t a. Foldable t => t a -> Bool
    let null_t_chirho = TyVarChirho(3060);
    let null_a_chirho = TyVarChirho(3061);
    env_chirho.bind_chirho(
        "null".to_string(),
        SchemeChirho {
            vars_chirho: vec![null_t_chirho, null_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Foldable".to_string(),
                ty_chirho: TyChirho::VarChirho(null_t_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(null_t_chirho)),
                    Box::new(TyChirho::VarChirho(null_a_chirho)),
                ),
                TyChirho::bool_chirho(),
            ),
        },
    );

    // length :: forall t a. Foldable t => t a -> Int
    let length_t_chirho = TyVarChirho(3070);
    let length_a_chirho = TyVarChirho(3071);
    env_chirho.bind_chirho(
        "length".to_string(),
        SchemeChirho {
            vars_chirho: vec![length_t_chirho, length_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Foldable".to_string(),
                ty_chirho: TyChirho::VarChirho(length_t_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(length_t_chirho)),
                    Box::new(TyChirho::VarChirho(length_a_chirho)),
                ),
                TyChirho::int_chirho(),
            ),
        },
    );

    // reverse :: forall a. [a] -> [a]
    let reverse_a_chirho = TyVarChirho(3080);
    env_chirho.bind_chirho(
        "reverse".to_string(),
        SchemeChirho {
            vars_chirho: vec![reverse_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(reverse_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(reverse_a_chirho))),
            ),
        },
    );
    env_chirho.bind_chirho(
        "Data.List.NonEmpty.reverse".to_string(),
        SchemeChirho {
            vars_chirho: vec![reverse_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("NonEmpty".to_string())),
                    Box::new(TyChirho::VarChirho(reverse_a_chirho)),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("NonEmpty".to_string())),
                    Box::new(TyChirho::VarChirho(reverse_a_chirho)),
                ),
            ),
        },
    );

    // zip :: forall a b. [a] -> [b] -> [(a, b)]
    let zip_a_chirho = TyVarChirho(3090);
    let zip_b_chirho = TyVarChirho(3091);
    env_chirho.bind_chirho(
        "zip".to_string(),
        SchemeChirho {
            vars_chirho: vec![zip_a_chirho, zip_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zip_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zip_b_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(zip_a_chirho),
                    TyChirho::VarChirho(zip_b_chirho),
                ]))),
            ),
        },
    );

    // zipWith :: forall a b c. (a -> b -> c) -> [a] -> [b] -> [c]
    let zipw_a_chirho = TyVarChirho(3100);
    let zipw_b_chirho = TyVarChirho(3101);
    let zipw_c_chirho = TyVarChirho(3102);
    env_chirho.bind_chirho(
        "zipWith".to_string(),
        SchemeChirho {
            vars_chirho: vec![zipw_a_chirho, zipw_b_chirho, zipw_c_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(zipw_a_chirho),
                            TyChirho::VarChirho(zipw_b_chirho),
                        ],
                        TyChirho::VarChirho(zipw_c_chirho),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipw_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipw_b_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(zipw_c_chirho))),
            ),
        },
    );

    // -----------------------------------------------------------------------
    // Additional Prelude list functions
    // -----------------------------------------------------------------------

    // (++) :: forall a. [a] -> [a] -> [a]
    let pp_a_chirho = TyVarChirho(3105);
    env_chirho.bind_chirho(
        "++".to_string(),
        SchemeChirho {
            vars_chirho: vec![pp_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(pp_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(pp_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(pp_a_chirho))),
            ),
        },
    );

    // append :: forall a. [a] -> [a] -> [a]
    let append_a_chirho = TyVarChirho(3110);
    env_chirho.bind_chirho(
        "append".to_string(),
        SchemeChirho {
            vars_chirho: vec![append_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_a_chirho))),
            ),
        },
    );

    // any :: forall a. (a -> Bool) -> [a] -> Bool
    let any_a_chirho = TyVarChirho(3120);
    env_chirho.bind_chirho(
        "any".to_string(),
        SchemeChirho {
            vars_chirho: vec![any_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(any_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(any_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // all :: forall a. (a -> Bool) -> [a] -> Bool
    let all_a_chirho = TyVarChirho(3130);
    env_chirho.bind_chirho(
        "all".to_string(),
        SchemeChirho {
            vars_chirho: vec![all_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(all_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(all_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // sum :: Num a => [a] -> a
    {
        let sum_a_chirho = TyVarChirho(3498);
        env_chirho.bind_chirho(
            "sum".to_string(),
            SchemeChirho {
                vars_chirho: vec![sum_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(sum_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sum_a_chirho))),
                    TyChirho::VarChirho(sum_a_chirho),
                ),
            },
        );
    }

    // product :: Num a => [a] -> a
    {
        let product_a_chirho = TyVarChirho(3499);
        env_chirho.bind_chirho(
            "product".to_string(),
            SchemeChirho {
                vars_chirho: vec![product_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(product_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(product_a_chirho))),
                    TyChirho::VarChirho(product_a_chirho),
                ),
            },
        );
    }

    // concatMap :: forall a b. (a -> [b]) -> [a] -> [b]
    let cm_a_chirho = TyVarChirho(3140);
    let cm_b_chirho = TyVarChirho(3141);
    env_chirho.bind_chirho(
        "concatMap".to_string(),
        SchemeChirho {
            vars_chirho: vec![cm_a_chirho, cm_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(cm_a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_b_chirho))),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cm_b_chirho))),
            ),
        },
    );

    // last :: forall a. [a] -> a
    let last_a_chirho = TyVarChirho(3150);
    env_chirho.bind_chirho(
        "last".to_string(),
        SchemeChirho {
            vars_chirho: vec![last_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(last_a_chirho))),
                TyChirho::VarChirho(last_a_chirho),
            ),
        },
    );

    // init :: forall a. [a] -> [a]
    let init_a_chirho = TyVarChirho(3160);
    env_chirho.bind_chirho(
        "init".to_string(),
        SchemeChirho {
            vars_chirho: vec![init_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(init_a_chirho))),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(init_a_chirho))),
            ),
        },
    );

    // elem :: Eq a => a -> [a] -> Bool (row 603: GHC's constraint, so the checker
    // records the element's Eq evidence)
    let elem_a_chirho = TyVarChirho(3500);
    env_chirho.bind_chirho(
        "elem".to_string(),
        SchemeChirho {
            vars_chirho: vec![elem_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Eq".to_string(),
                ty_chirho: TyChirho::VarChirho(elem_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(elem_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(elem_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // notElem :: Eq a => a -> [a] -> Bool (row 603)
    let notelem_a_chirho = TyVarChirho(3501);
    env_chirho.bind_chirho(
        "notElem".to_string(),
        SchemeChirho {
            vars_chirho: vec![notelem_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Eq".to_string(),
                ty_chirho: TyChirho::VarChirho(notelem_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(notelem_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(notelem_a_chirho))),
                ],
                TyChirho::bool_chirho(),
            ),
        },
    );

    // minimum :: Ord a => [a] -> a
    {
        let minimum_a_chirho = TyVarChirho(3502);
        env_chirho.bind_chirho(
            "minimum".to_string(),
            SchemeChirho {
                vars_chirho: vec![minimum_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(minimum_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(minimum_a_chirho))),
                    TyChirho::VarChirho(minimum_a_chirho),
                ),
            },
        );
    }

    // maximum :: Ord a => [a] -> a
    {
        let maximum_a_chirho = TyVarChirho(3503);
        env_chirho.bind_chirho(
            "maximum".to_string(),
            SchemeChirho {
                vars_chirho: vec![maximum_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(maximum_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(maximum_a_chirho))),
                    TyChirho::VarChirho(maximum_a_chirho),
                ),
            },
        );
    }

    // sort :: forall a. Ord a => [a] -> [a]
    {
        let sort_a_chirho = TyVarChirho(3088);
        env_chirho.bind_chirho(
            "sort".to_string(),
            SchemeChirho {
                vars_chirho: vec![sort_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(sort_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sort_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sort_a_chirho))),
                ),
            },
        );
    }

    // insert :: Int -> [Int] -> [Int]  (insertion for sort)
    env_chirho.bind_chirho(
        "insert".to_string(),
        SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::int_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            ),
        },
    );

    // abs :: forall a. Num a => a -> a
    {
        let abs_v_chirho = TyVarChirho(2210);
        env_chirho.bind_chirho(
            "abs".to_string(),
            SchemeChirho {
                vars_chirho: vec![abs_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(abs_v_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(abs_v_chirho),
                    TyChirho::VarChirho(abs_v_chirho),
                ),
            },
        );
    }

    // signum :: forall a. Num a => a -> a
    {
        let sig_v_chirho = TyVarChirho(2211);
        env_chirho.bind_chirho(
            "signum".to_string(),
            SchemeChirho {
                vars_chirho: vec![sig_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(sig_v_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(sig_v_chirho),
                    TyChirho::VarChirho(sig_v_chirho),
                ),
            },
        );
    }

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

    // gcd, lcm :: forall a. Integral a => a -> a -> a
    // Their generated Prelude bodies already preserve the operand type. The
    // schemes keep that proof visible to downstream occurrence evidence too.
    {
        let integral_v_chirho = TyVarChirho(2212);
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![integral_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Integral".to_string(),
                ty_chirho: TyChirho::VarChirho(integral_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(integral_v_chirho),
                    TyChirho::VarChirho(integral_v_chirho),
                ],
                TyChirho::VarChirho(integral_v_chirho),
            ),
        };
        env_chirho.bind_chirho("gcd".to_string(), scheme_chirho.clone());
        env_chirho.bind_chirho("lcm".to_string(), scheme_chirho);
    }
}
