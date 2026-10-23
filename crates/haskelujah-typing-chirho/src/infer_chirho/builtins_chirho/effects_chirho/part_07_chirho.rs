// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 07 of 14: original lines 13475-14431 of
//! `seed_builtins_chirho`, in order (56 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::infer_chirho::map_ty_chirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_07_chirho(env_chirho: &mut TyEnvChirho) {
    // Data.Map.mapWithKey :: (k -> v -> w) -> Map k v -> Map k w
    {
        let k_chirho = TyVarChirho(38034);
        let v_chirho = TyVarChirho(38035);
        let w_chirho = TyVarChirho(38036);
        let input_map_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(v_chirho)),
        );
        let result_map_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Map".to_string())),
                Box::new(TyChirho::VarChirho(k_chirho)),
            )),
            Box::new(TyChirho::VarChirho(w_chirho)),
        );
        env_chirho.bind_chirho(
            "Data.Map.mapWithKey".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho, w_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(k_chirho),
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(v_chirho),
                                TyChirho::VarChirho(w_chirho),
                            ),
                        ),
                        input_map_ty_chirho,
                    ],
                    result_map_ty_chirho,
                ),
            },
        );
    }

    // Data.Map.keysSet :: Map k v -> Set k
    {
        let k_chirho = TyVarChirho(38037);
        let v_chirho = TyVarChirho(38038);
        env_chirho.bind_chirho(
            "Data.Map.keysSet".to_string(),
            SchemeChirho {
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
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Set".to_string())),
                        Box::new(TyChirho::VarChirho(k_chirho)),
                    ),
                ),
            },
        );
    }

    // Data.Map.foldr :: (v -> b -> b) -> b -> Map k v -> b
    {
        let k_chirho = TyVarChirho(38039);
        let v_chirho = TyVarChirho(38040);
        let b_chirho = TyVarChirho(38041);
        env_chirho.bind_chirho(
            "Data.Map.foldr".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(v_chirho),
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(b_chirho),
                                TyChirho::VarChirho(b_chirho),
                            ),
                        ),
                        TyChirho::VarChirho(b_chirho),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::AppChirho(
                                Box::new(TyChirho::ConChirho("Map".to_string())),
                                Box::new(TyChirho::VarChirho(k_chirho)),
                            )),
                            Box::new(TyChirho::VarChirho(v_chirho)),
                        ),
                    ],
                    TyChirho::VarChirho(b_chirho),
                ),
            },
        );
    }

    // Data.Map.findMin / Data.Map.findMax :: Map k v -> (k, v)
    {
        let k_chirho = TyVarChirho(38042);
        let v_chirho = TyVarChirho(38043);
        let map_find_extrema_scheme_chirho = SchemeChirho {
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
                TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(k_chirho),
                    TyChirho::VarChirho(v_chirho),
                ]),
            ),
        };
        env_chirho.bind_chirho(
            "Data.Map.findMin".to_string(),
            map_find_extrema_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Data.Map.findMax".to_string(),
            map_find_extrema_scheme_chirho,
        );
    }

    // mapFoldlWithKey :: (b -> k -> v -> b) -> b -> Map k v -> b
    {
        let k_chirho = TyChirho::VarChirho(TyVarChirho(3860));
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3226));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3227));
        env_chirho.bind_chirho(
            "mapFoldlWithKey".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3226), TyVarChirho(3227), TyVarChirho(3860)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![b_chirho.clone(), k_chirho.clone(), v_chirho.clone()],
                            b_chirho.clone(),
                        ),
                        b_chirho.clone(),
                        map_ty_chirho(k_chirho, v_chirho),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // mapFoldrWithKey :: (k -> v -> b -> b) -> b -> Map k v -> b
    {
        let k_chirho = TyChirho::VarChirho(TyVarChirho(3861));
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3840));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3841));
        env_chirho.bind_chirho(
            "mapFoldrWithKey".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3840), TyVarChirho(3841), TyVarChirho(3861)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![k_chirho.clone(), v_chirho.clone(), b_chirho.clone()],
                            b_chirho.clone(),
                        ),
                        b_chirho.clone(),
                        map_ty_chirho(k_chirho, v_chirho),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // mapFoldlWithKey' :: (b -> k -> v -> b) -> b -> Map k v -> b
    {
        let k_chirho = TyChirho::VarChirho(TyVarChirho(3862));
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3845));
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3846));
        env_chirho.bind_chirho(
            "mapFoldlWithKey'".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3845), TyVarChirho(3846), TyVarChirho(3862)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![b_chirho.clone(), k_chirho.clone(), v_chirho.clone()],
                            b_chirho.clone(),
                        ),
                        b_chirho.clone(),
                        map_ty_chirho(k_chirho, v_chirho),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // mapInsertWith :: Ord k => (v -> v -> v) -> k -> v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3805);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3230));
        env_chirho.bind_chirho(
            "mapInsertWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, TyVarChirho(3230)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            v_chirho.clone(),
                            TyChirho::fun_chirho(v_chirho.clone(), v_chirho.clone()),
                        ),
                        TyChirho::VarChirho(k_chirho),
                        v_chirho.clone(),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho),
                ),
            },
        );
    }

    // mapFindWithDefault :: Ord k => v -> k -> Map k v -> v
    {
        let k_chirho = TyVarChirho(3806);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3231));
        env_chirho.bind_chirho(
            "mapFindWithDefault".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, TyVarChirho(3231)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        v_chirho.clone(),
                        TyChirho::VarChirho(k_chirho),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                    ],
                    v_chirho,
                ),
            },
        );
    }

    // mapAdjust :: Ord k => (v -> v) -> k -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3807);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3232));
        env_chirho.bind_chirho(
            "mapAdjust".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, TyVarChirho(3232)],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho.clone(), v_chirho.clone()),
                        TyChirho::VarChirho(k_chirho),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho),
                ),
            },
        );
    }

    // mapUnionWith :: Ord k => (v -> v -> v) -> Map k v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3863);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3233));
        env_chirho.bind_chirho(
            "mapUnionWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3233), k_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            v_chirho.clone(),
                            TyChirho::fun_chirho(v_chirho.clone(), v_chirho.clone()),
                        ),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho),
                ),
            },
        );
    }

    // mapUnion :: Ord k => Map k v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3864);
        let v_chirho = TyVarChirho(3865);
        env_chirho.bind_chirho(
            "mapUnion".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho)),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho)),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho)),
                ),
            },
        );
    }

    // mapDifference :: Ord k => Map k v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3866);
        let v_chirho = TyVarChirho(3867);
        env_chirho.bind_chirho(
            "mapDifference".to_string(),
            SchemeChirho {
                vars_chirho: vec![k_chirho, v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho)),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho)),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), TyChirho::VarChirho(v_chirho)),
                ),
            },
        );
    }

    // mapIntersectionWith :: Ord k => (v -> v -> v) -> Map k v -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3868);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3234));
        env_chirho.bind_chirho(
            "mapIntersectionWith".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3234), k_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(k_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            v_chirho.clone(),
                            TyChirho::fun_chirho(v_chirho.clone(), v_chirho.clone()),
                        ),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho),
                ),
            },
        );
    }

    // mapFilter :: (v -> Bool) -> Map k v -> Map k v
    {
        let k_chirho = TyVarChirho(3869);
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3235));
        env_chirho.bind_chirho(
            "mapFilter".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3235), k_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(v_chirho.clone(), TyChirho::bool_chirho()),
                        map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho.clone()),
                    ],
                    map_ty_chirho(TyChirho::VarChirho(k_chirho), v_chirho),
                ),
            },
        );
    }

    // setEmpty :: Set Int  (represented as Int placeholder)
    env_chirho.bind_chirho(
        "setEmpty".to_string(),
        SchemeChirho::mono_chirho(TyChirho::int_chirho()),
    );

    // setSingleton :: Int -> Set Int
    env_chirho.bind_chirho(
        "setSingleton".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // setInsert :: Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setInsert".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setMember :: Int -> Set Int -> Bool
    env_chirho.bind_chirho(
        "setMember".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::bool_chirho(),
        )),
    );

    // setSize :: Set Int -> Int
    env_chirho.bind_chirho(
        "setSize".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // setToList :: Set Int -> [Int]
    env_chirho.bind_chirho(
        "setToList".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
        )),
    );

    // setFromList :: [Int] -> Set Int
    env_chirho.bind_chirho(
        "setFromList".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::int_chirho())),
            TyChirho::int_chirho(),
        )),
    );

    // setDelete :: Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setDelete".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setUnion :: Set Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setUnion".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setIntersection :: Set Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setIntersection".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setDifference :: Set Int -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setDifference".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::int_chirho(),
        )),
    );

    // setFilter :: (Int -> Bool) -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setFilter".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::bool_chirho()),
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
        )),
    );

    // setMap :: (Int -> Int) -> Set Int -> Set Int
    env_chirho.bind_chirho(
        "setMap".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
            TyChirho::fun_chirho(TyChirho::int_chirho(), TyChirho::int_chirho()),
        )),
    );

    // setFold :: (Int -> b -> b) -> b -> Set Int -> b
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3230));
        env_chirho.bind_chirho(
            "setFold".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3230)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::int_chirho(),
                            TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone()),
                        ),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // setFoldr :: (Int -> b -> b) -> b -> Set Int -> b (alias for setFold)
    {
        let b_chirho = TyChirho::VarChirho(TyVarChirho(3231));
        env_chirho.bind_chirho(
            "setFoldr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3231)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::int_chirho(),
                            TyChirho::fun_chirho(b_chirho.clone(), b_chirho.clone()),
                        ),
                        b_chirho.clone(),
                        TyChirho::int_chirho(),
                    ],
                    b_chirho,
                ),
            },
        );
    }

    // when :: forall f. Applicative f => Bool -> f () -> f ()
    let when_f_chirho = TyVarChirho(3498);
    let when_scheme_chirho = SchemeChirho {
        vars_chirho: vec![when_f_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Applicative".to_string(),
            ty_chirho: TyChirho::VarChirho(when_f_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::bool_chirho(),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(when_f_chirho)),
                    Box::new(TyChirho::unit_chirho()),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(when_f_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    env_chirho.bind_chirho("when".to_string(), when_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.when".to_string(), when_scheme_chirho);

    // unless :: forall f. Applicative f => Bool -> f () -> f ()
    let unless_f_chirho = TyVarChirho(3499);
    let unless_scheme_chirho = SchemeChirho {
        vars_chirho: vec![unless_f_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Applicative".to_string(),
            ty_chirho: TyChirho::VarChirho(unless_f_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::bool_chirho(),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(unless_f_chirho)),
                    Box::new(TyChirho::unit_chirho()),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(unless_f_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    env_chirho.bind_chirho("unless".to_string(), unless_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.unless".to_string(), unless_scheme_chirho);

    // mapM_ :: forall t m a b. (Foldable t, Monad m) => (a -> m b) -> t a -> m ()
    let mapm_t_chirho = TyVarChirho(3500);
    let mapm_m_chirho = TyVarChirho(3501);
    let mapm_a_chirho = TyVarChirho(3502);
    let mapm_b_chirho = TyVarChirho(3503);
    let mapm_scheme_chirho = SchemeChirho {
        vars_chirho: vec![mapm_t_chirho, mapm_m_chirho, mapm_a_chirho, mapm_b_chirho],
        preds_chirho: vec![
            SchemePredChirho {
                class_name_chirho: "Foldable".to_string(),
                ty_chirho: TyChirho::VarChirho(mapm_t_chirho),
                extra_tys_chirho: vec![],
            },
            SchemePredChirho {
                class_name_chirho: "Monad".to_string(),
                ty_chirho: TyChirho::VarChirho(mapm_m_chirho),
                extra_tys_chirho: vec![],
            },
        ],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(mapm_a_chirho),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(mapm_m_chirho)),
                        Box::new(TyChirho::VarChirho(mapm_b_chirho)),
                    ),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(mapm_t_chirho)),
                    Box::new(TyChirho::VarChirho(mapm_a_chirho)),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(mapm_m_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    for name_chirho in ["mapM_", "Control.Monad.mapM_", "Data.Foldable.mapM_"] {
        env_chirho.bind_chirho(name_chirho.to_string(), mapm_scheme_chirho.clone());
    }

    // forM_ :: forall t m a b. (Foldable t, Monad m) => t a -> (a -> m b) -> m ()
    let form_t_chirho = TyVarChirho(3504);
    let form_m_chirho = TyVarChirho(3505);
    let form_a_chirho = TyVarChirho(3506);
    let form_b_chirho = TyVarChirho(3507);
    let form_scheme_chirho = SchemeChirho {
        vars_chirho: vec![form_t_chirho, form_m_chirho, form_a_chirho, form_b_chirho],
        preds_chirho: vec![
            SchemePredChirho {
                class_name_chirho: "Foldable".to_string(),
                ty_chirho: TyChirho::VarChirho(form_t_chirho),
                extra_tys_chirho: vec![],
            },
            SchemePredChirho {
                class_name_chirho: "Monad".to_string(),
                ty_chirho: TyChirho::VarChirho(form_m_chirho),
                extra_tys_chirho: vec![],
            },
        ],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(form_t_chirho)),
                    Box::new(TyChirho::VarChirho(form_a_chirho)),
                ),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(form_a_chirho),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(form_m_chirho)),
                        Box::new(TyChirho::VarChirho(form_b_chirho)),
                    ),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(form_m_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    for name_chirho in ["forM_", "Control.Monad.forM_", "Data.Foldable.forM_"] {
        env_chirho.bind_chirho(name_chirho.to_string(), form_scheme_chirho.clone());
    }

    // putChar :: Char -> IO ()
    env_chirho.bind_chirho(
        "putChar".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // sequence_ :: forall t m a. (Foldable t, Monad m) => t (m a) -> m ()
    let sequence_t_chirho = TyVarChirho(3510);
    let sequence_m_chirho = TyVarChirho(3511);
    let sequence_a_chirho = TyVarChirho(3512);
    let sequence_scheme_chirho = SchemeChirho {
        vars_chirho: vec![sequence_t_chirho, sequence_m_chirho, sequence_a_chirho],
        preds_chirho: vec![
            SchemePredChirho {
                class_name_chirho: "Foldable".to_string(),
                ty_chirho: TyChirho::VarChirho(sequence_t_chirho),
                extra_tys_chirho: vec![],
            },
            SchemePredChirho {
                class_name_chirho: "Monad".to_string(),
                ty_chirho: TyChirho::VarChirho(sequence_m_chirho),
                extra_tys_chirho: vec![],
            },
        ],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(sequence_t_chirho)),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(sequence_m_chirho)),
                    Box::new(TyChirho::VarChirho(sequence_a_chirho)),
                )),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(sequence_m_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    for name_chirho in [
        "sequence_",
        "Control.Monad.sequence_",
        "Data.Foldable.sequence_",
    ] {
        env_chirho.bind_chirho(name_chirho.to_string(), sequence_scheme_chirho.clone());
    }

    // void :: forall f a. Functor f => f a -> f ()
    let void_f_chirho = TyVarChirho(3512);
    let void_a_chirho = TyVarChirho(3513);
    let void_scheme_chirho = SchemeChirho {
        vars_chirho: vec![void_f_chirho, void_a_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Functor".to_string(),
            ty_chirho: TyChirho::VarChirho(void_f_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(void_f_chirho)),
                Box::new(TyChirho::VarChirho(void_a_chirho)),
            ),
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(void_f_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    env_chirho.bind_chirho("void".to_string(), void_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.void".to_string(), void_scheme_chirho.clone());
    env_chirho.bind_chirho("Data.Functor.void".to_string(), void_scheme_chirho);

    // guard :: forall f. Alternative f => Bool -> f ()
    let guard_f_chirho = TyVarChirho(3514);
    let guard_scheme_chirho = SchemeChirho {
        vars_chirho: vec![guard_f_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Alternative".to_string(),
            ty_chirho: TyChirho::VarChirho(guard_f_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::bool_chirho(),
            TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(guard_f_chirho)),
                Box::new(TyChirho::unit_chirho()),
            ),
        ),
    };
    env_chirho.bind_chirho("guard".to_string(), guard_scheme_chirho.clone());
    env_chirho.bind_chirho("Control.Monad.guard".to_string(), guard_scheme_chirho);

    // interact :: (String -> String) -> IO ()
    env_chirho.bind_chirho(
        "interact".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::fun_chirho(TyChirho::string_chirho(), TyChirho::string_chirho()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // Test.Tasty / Test.Tasty.Options
    {
        let test_tree_ty_chirho = TyChirho::ConChirho("TestTree".to_string());
        let ingredient_ty_chirho = TyChirho::ConChirho("Ingredient".to_string());
        let option_description_ty_chirho = TyChirho::ConChirho("OptionDescription".to_string());
        let option_set_ty_chirho = TyChirho::ConChirho("OptionSet".to_string());

        env_chirho.bind_chirho(
            "Test.Tasty.defaultMain".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                test_tree_ty_chirho.clone(),
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            )),
        );
        env_chirho.bind_chirho(
            "Test.Tasty.defaultMainWithIngredients".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(ingredient_ty_chirho.clone())),
                    test_tree_ty_chirho.clone(),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            )),
        );
        env_chirho.bind_chirho(
            "Test.Tasty.testGroup".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![
                    TyChirho::string_chirho(),
                    TyChirho::ListChirho(Box::new(test_tree_ty_chirho.clone())),
                ],
                test_tree_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            "Test.Tasty.includingOptions".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(option_description_ty_chirho.clone())),
                ingredient_ty_chirho,
            )),
        );

        let option_a_chirho = TyVarChirho(71351);
        env_chirho.bind_chirho(
            "Test.Tasty.Options.Option".to_string(),
            SchemeChirho {
                vars_chirho: vec![option_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Proxy".to_string())),
                        Box::new(TyChirho::VarChirho(option_a_chirho)),
                    ),
                    option_description_ty_chirho,
                ),
            },
        );
        env_chirho.bind_chirho(
            "Test.Tasty.Options.lookupOption".to_string(),
            SchemeChirho {
                vars_chirho: vec![option_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    option_set_ty_chirho,
                    TyChirho::VarChirho(option_a_chirho),
                ),
            },
        );
        env_chirho.bind_chirho(
            "Test.Tasty.Options.defaultValue".to_string(),
            SchemeChirho {
                vars_chirho: vec![option_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::VarChirho(option_a_chirho),
            },
        );
        env_chirho.bind_chirho(
            "Test.Tasty.Options.parseValue".to_string(),
            SchemeChirho {
                vars_chirho: vec![option_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(option_a_chirho)),
                    ),
                ),
            },
        );
        let maybe_string_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::string_chirho()),
        );
        env_chirho.bind_chirho(
            "Test.Tasty.Options.optionName".to_string(),
            SchemeChirho::mono_chirho(maybe_string_ty_chirho.clone()),
        );
        env_chirho.bind_chirho(
            "Test.Tasty.Options.optionHelp".to_string(),
            SchemeChirho::mono_chirho(maybe_string_ty_chirho),
        );
    }

    // lines :: String -> [String]
    env_chirho.bind_chirho(
        "lines".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
        )),
    );

    // unlines :: [String] -> String
    env_chirho.bind_chirho(
        "unlines".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::string_chirho())),
            TyChirho::string_chirho(),
        )),
    );

    // error :: forall a. String -> a
    let error_v_chirho = TyVarChirho(3400);
    env_chirho.bind_chirho(
        "error".to_string(),
        SchemeChirho {
            vars_chirho: vec![error_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::VarChirho(error_v_chirho),
            ),
        },
    );

    // undefined :: forall a. a
    let undef_v_chirho = TyVarChirho(3401);
    env_chirho.bind_chirho(
        "undefined".to_string(),
        SchemeChirho {
            vars_chirho: vec![undef_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::VarChirho(undef_v_chirho),
        },
    );
}
