// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 09 of 14: original lines 15384-16316 of
//! `seed_builtins_chirho`, in order (41 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::infer_chirho::map_ty_chirho;
use crate::ty_chirho::{MultChirho, SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_09_chirho(env_chirho: &mut TyEnvChirho) {
    // replicate :: forall a. Int -> a -> [a]
    let rep_a_chirho = TyVarChirho(3250);
    env_chirho.bind_chirho(
        "replicate".to_string(),
        SchemeChirho {
            vars_chirho: vec![rep_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), TyChirho::VarChirho(rep_a_chirho)],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(rep_a_chirho))),
            ),
        },
    );

    // takeWhile :: forall a. (a -> Bool) -> [a] -> [a]
    let tw_a_chirho = TyVarChirho(3300);
    env_chirho.bind_chirho(
        "takeWhile".to_string(),
        SchemeChirho {
            vars_chirho: vec![tw_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(tw_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tw_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tw_a_chirho))),
            ),
        },
    );

    // dropWhile :: forall a. (a -> Bool) -> [a] -> [a]
    let dw_a_chirho = TyVarChirho(3301);
    env_chirho.bind_chirho(
        "dropWhile".to_string(),
        SchemeChirho {
            vars_chirho: vec![dw_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(TyChirho::VarChirho(dw_a_chirho), TyChirho::bool_chirho()),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(dw_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(dw_a_chirho))),
            ),
        },
    );

    // iterate :: forall a. (a -> a) -> a -> [a]
    let iter_a_chirho = TyVarChirho(3302);
    env_chirho.bind_chirho(
        "iterate".to_string(),
        SchemeChirho {
            vars_chirho: vec![iter_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(iter_a_chirho),
                        TyChirho::VarChirho(iter_a_chirho),
                    ),
                    TyChirho::VarChirho(iter_a_chirho),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(iter_a_chirho))),
            ),
        },
    );

    // lookup :: Eq a => a -> [(a,b)] -> Maybe b (row 603)
    let lu_a_chirho = TyVarChirho(3303);
    let lu_b_chirho = TyVarChirho(3304);
    env_chirho.bind_chirho(
        "lookup".to_string(),
        SchemeChirho {
            vars_chirho: vec![lu_a_chirho, lu_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Eq".to_string(),
                ty_chirho: TyChirho::VarChirho(lu_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(lu_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(lu_a_chirho),
                        TyChirho::VarChirho(lu_b_chirho),
                    ]))),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(lu_b_chirho)),
                ),
            ),
        },
    );

    // unzip :: forall a b. [(a,b)] -> ([a],[b])
    let uz_a_chirho = TyVarChirho(3305);
    let uz_b_chirho = TyVarChirho(3306);
    env_chirho.bind_chirho(
        "unzip".to_string(),
        SchemeChirho {
            vars_chirho: vec![uz_a_chirho, uz_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(uz_a_chirho),
                    TyChirho::VarChirho(uz_b_chirho),
                ]))),
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz_b_chirho))),
                ]),
            ),
        },
    );

    // scanl :: forall a b. (b -> a -> b) -> b -> [a] -> [b]
    let sl_a_chirho = TyVarChirho(3307);
    let sl_b_chirho = TyVarChirho(3308);
    env_chirho.bind_chirho(
        "scanl".to_string(),
        SchemeChirho {
            vars_chirho: vec![sl_a_chirho, sl_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_n_chirho(
                        vec![
                            TyChirho::VarChirho(sl_b_chirho),
                            TyChirho::VarChirho(sl_a_chirho),
                        ],
                        TyChirho::VarChirho(sl_b_chirho),
                    ),
                    TyChirho::VarChirho(sl_b_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sl_a_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(sl_b_chirho))),
            ),
        },
    );

    // span :: forall a. (a -> Bool) -> [a] -> ([a],[a])
    let span_a_chirho = TyVarChirho(3309);
    env_chirho.bind_chirho(
        "span".to_string(),
        SchemeChirho {
            vars_chirho: vec![span_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(span_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(span_a_chirho))),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(span_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(span_a_chirho))),
                ]),
            ),
        },
    );

    // break :: forall a. (a -> Bool) -> [a] -> ([a],[a])
    let break_a_chirho = TyVarChirho(3310);
    env_chirho.bind_chirho(
        "break".to_string(),
        SchemeChirho {
            vars_chirho: vec![break_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(break_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(break_a_chirho))),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(break_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(break_a_chirho))),
                ]),
            ),
        },
    );

    // partition :: forall a. (a -> Bool) -> [a] -> ([a],[a])
    let part_a_chirho = TyVarChirho(3311);
    env_chirho.bind_chirho(
        "partition".to_string(),
        SchemeChirho {
            vars_chirho: vec![part_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(part_a_chirho),
                        TyChirho::bool_chirho(),
                    ),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(part_a_chirho))),
                ],
                TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(part_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(part_a_chirho))),
                ]),
            ),
        },
    );

    // mapInsertStr :: String -> v -> Map String v -> Map String v
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3240));
        env_chirho.bind_chirho(
            "mapInsertStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3240)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::string_chirho(),
                        v_chirho.clone(),
                        map_ty_chirho(TyChirho::string_chirho(), v_chirho.clone()),
                    ],
                    map_ty_chirho(TyChirho::string_chirho(), v_chirho),
                ),
            },
        );
    }

    // mapLookupStr :: String -> Map String v -> v
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3241));
        env_chirho.bind_chirho(
            "mapLookupStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3241)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(
                        map_ty_chirho(TyChirho::string_chirho(), v_chirho.clone()),
                        v_chirho,
                    ),
                ),
            },
        );
    }

    // mapMemberStr :: String -> Map String v -> Bool
    {
        let v_chirho = TyVarChirho(3870);
        env_chirho.bind_chirho(
            "mapMemberStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![v_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(
                        map_ty_chirho(TyChirho::string_chirho(), TyChirho::VarChirho(v_chirho)),
                        TyChirho::bool_chirho(),
                    ),
                ),
            },
        );
    }

    // mapFindWithDefaultStr :: v -> String -> Map String v -> v
    {
        let v_chirho = TyChirho::VarChirho(TyVarChirho(3242));
        env_chirho.bind_chirho(
            "mapFindWithDefaultStr".to_string(),
            SchemeChirho {
                vars_chirho: vec![TyVarChirho(3242)],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        v_chirho.clone(),
                        TyChirho::string_chirho(),
                        map_ty_chirho(TyChirho::string_chirho(), v_chirho.clone()),
                    ],
                    v_chirho,
                ),
            },
        );
    }

    // nub :: [a] -> [a]
    {
        let nub_a_chirho = TyVarChirho(3400);
        env_chirho.bind_chirho(
            "nub".to_string(),
            SchemeChirho {
                vars_chirho: vec![nub_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nub_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(nub_a_chirho))),
                ),
            },
        );
    }

    // zip3 :: [a] -> [b] -> [c] -> [(a,b,c)]
    {
        let z3a_chirho = TyVarChirho(3401);
        let z3b_chirho = TyVarChirho(3402);
        let z3c_chirho = TyVarChirho(3403);
        env_chirho.bind_chirho(
            "zip3".to_string(),
            SchemeChirho {
                vars_chirho: vec![z3a_chirho, z3b_chirho, z3c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(z3a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(z3b_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(z3c_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(z3a_chirho),
                        TyChirho::VarChirho(z3b_chirho),
                        TyChirho::VarChirho(z3c_chirho),
                    ]))),
                ),
            },
        );
    }

    // intersperse :: a -> [a] -> [a]
    {
        let isp_a_chirho = TyVarChirho(3404);
        env_chirho.bind_chirho(
            "intersperse".to_string(),
            SchemeChirho {
                vars_chirho: vec![isp_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(isp_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isp_a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isp_a_chirho))),
                    ),
                ),
            },
        );
    }

    // isPrefixOf :: Eq a => [a] -> [a] -> Bool (row 603)
    {
        let ipf_a_chirho = TyVarChirho(3405);
        env_chirho.bind_chirho(
            "isPrefixOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![ipf_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(ipf_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ipf_a_chirho))),
                    TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(ipf_a_chirho))),
                        TyChirho::bool_chirho(),
                    ),
                ),
            },
        );
    }

    // isSuffixOf :: Eq a => [a] -> [a] -> Bool (row 603)
    {
        let isf_a_chirho = TyVarChirho(3406);
        env_chirho.bind_chirho(
            "isSuffixOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![isf_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(isf_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isf_a_chirho))),
                    TyChirho::fun_chirho(
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(isf_a_chirho))),
                        TyChirho::bool_chirho(),
                    ),
                ),
            },
        );
    }

    // unzip3 :: [(a,b,c)] -> ([a],[b],[c])
    {
        let uz3a_chirho = TyVarChirho(3407);
        let uz3b_chirho = TyVarChirho(3408);
        let uz3c_chirho = TyVarChirho(3409);
        env_chirho.bind_chirho(
            "unzip3".to_string(),
            SchemeChirho {
                vars_chirho: vec![uz3a_chirho, uz3b_chirho, uz3c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(uz3a_chirho),
                        TyChirho::VarChirho(uz3b_chirho),
                        TyChirho::VarChirho(uz3c_chirho),
                    ]))),
                    TyChirho::TupleChirho(vec![
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz3a_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz3b_chirho))),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(uz3c_chirho))),
                    ]),
                ),
            },
        );
    }

    // toEnum :: forall a. Enum a => Int -> a
    let enum_a_chirho = TyVarChirho(3260);
    env_chirho.bind_chirho(
        "toEnum".to_string(),
        SchemeChirho {
            vars_chirho: vec![enum_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(enum_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::VarChirho(enum_a_chirho),
            ),
        },
    );

    // fromEnum :: forall a. Enum a => a -> Int
    let from_enum_a_chirho = TyVarChirho(3261);
    env_chirho.bind_chirho(
        "fromEnum".to_string(),
        SchemeChirho {
            vars_chirho: vec![from_enum_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(from_enum_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(from_enum_a_chirho),
                TyChirho::int_chirho(),
            ),
        },
    );

    // succ :: forall a. Enum a => a -> a
    let succ_a_chirho = TyVarChirho(3262);
    env_chirho.bind_chirho(
        "succ".to_string(),
        SchemeChirho {
            vars_chirho: vec![succ_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(succ_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(succ_a_chirho),
                TyChirho::VarChirho(succ_a_chirho),
            ),
        },
    );

    // pred :: forall a. Enum a => a -> a
    let pred_a_chirho = TyVarChirho(3263);
    env_chirho.bind_chirho(
        "pred".to_string(),
        SchemeChirho {
            vars_chirho: vec![pred_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(pred_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(pred_a_chirho),
                TyChirho::VarChirho(pred_a_chirho),
            ),
        },
    );

    // enumFrom :: forall a. Enum a => a -> [a]
    let enum_from_a_chirho = TyVarChirho(3266);
    env_chirho.bind_chirho(
        "enumFrom".to_string(),
        SchemeChirho {
            vars_chirho: vec![enum_from_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(enum_from_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(enum_from_a_chirho),
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(enum_from_a_chirho))),
            ),
        },
    );

    // enumFromThen :: forall a. Enum a => a -> a -> [a]
    let enum_from_then_a_chirho = TyVarChirho(3267);
    env_chirho.bind_chirho(
        "enumFromThen".to_string(),
        SchemeChirho {
            vars_chirho: vec![enum_from_then_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(enum_from_then_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(enum_from_then_a_chirho),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(enum_from_then_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(enum_from_then_a_chirho))),
                ),
            ),
        },
    );

    // enumFromTo :: forall a. Enum a => a -> a -> [a]
    let enum_from_to_a_chirho = TyVarChirho(3268);
    env_chirho.bind_chirho(
        "enumFromTo".to_string(),
        SchemeChirho {
            vars_chirho: vec![enum_from_to_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(enum_from_to_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(enum_from_to_a_chirho),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(enum_from_to_a_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(enum_from_to_a_chirho))),
                ),
            ),
        },
    );

    // enumFromThenTo :: forall a. Enum a => a -> a -> a -> [a]
    let enum_from_then_to_a_chirho = TyVarChirho(3269);
    env_chirho.bind_chirho(
        "enumFromThenTo".to_string(),
        SchemeChirho {
            vars_chirho: vec![enum_from_then_to_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Enum".to_string(),
                ty_chirho: TyChirho::VarChirho(enum_from_then_to_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(enum_from_then_to_a_chirho),
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(enum_from_then_to_a_chirho),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(enum_from_then_to_a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                            enum_from_then_to_a_chirho,
                        ))),
                    ),
                ),
            ),
        },
    );

    // minBound :: forall a. Bounded a => a
    let min_bound_a_chirho = TyVarChirho(3264);
    env_chirho.bind_chirho(
        "minBound".to_string(),
        SchemeChirho {
            vars_chirho: vec![min_bound_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bounded".to_string(),
                ty_chirho: TyChirho::VarChirho(min_bound_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::VarChirho(min_bound_a_chirho),
        },
    );

    // maxBound :: forall a. Bounded a => a
    let max_bound_a_chirho = TyVarChirho(3265);
    env_chirho.bind_chirho(
        "maxBound".to_string(),
        SchemeChirho {
            vars_chirho: vec![max_bound_a_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bounded".to_string(),
                ty_chirho: TyChirho::VarChirho(max_bound_a_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::VarChirho(max_bound_a_chirho),
        },
    );

    // Data.Char functions
    let char_ty_chirho = TyChirho::ConChirho("Char".to_string());

    // chr :: Int -> Char
    env_chirho.bind_chirho(
        "chr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            char_ty_chirho.clone(),
        )),
    );
    for name_chirho in ["unsafeChr", "GHC.Base.unsafeChr", "GHC.Char.unsafeChr"] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                char_ty_chirho.clone(),
            )),
        );
    }

    // ord :: Char -> Int
    env_chirho.bind_chirho(
        "ord".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            char_ty_chirho.clone(),
            TyChirho::int_chirho(),
        )),
    );

    // Char -> Bool functions
    for name_chirho in &[
        "isDigit",
        "isAlpha",
        "isAlphaNum",
        "isUpper",
        "isLower",
        "isSpace",
    ] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                char_ty_chirho.clone(),
                TyChirho::bool_chirho(),
            )),
        );
    }

    // Char -> Char functions
    for name_chirho in &["toLower", "toUpper"] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                char_ty_chirho.clone(),
                char_ty_chirho.clone(),
            )),
        );
    }

    // digitToInt :: Char -> Int
    env_chirho.bind_chirho(
        "digitToInt".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(char_ty_chirho, TyChirho::int_chirho())),
    );

    // generalCategory :: Char -> GeneralCategory
    env_chirho.bind_chirho(
        "generalCategory".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Char".to_string()),
            TyChirho::ConChirho("GeneralCategory".to_string()),
        )),
    );

    for general_category_ctor_chirho in &[
        "UppercaseLetter",
        "LowercaseLetter",
        "TitlecaseLetter",
        "ModifierLetter",
        "OtherLetter",
        "NonSpacingMark",
        "SpacingCombiningMark",
        "EnclosingMark",
        "DecimalNumber",
        "LetterNumber",
        "OtherNumber",
        "ConnectorPunctuation",
        "DashPunctuation",
        "OpenPunctuation",
        "ClosePunctuation",
        "InitialQuote",
        "FinalQuote",
        "OtherPunctuation",
        "MathSymbol",
        "CurrencySymbol",
        "ModifierSymbol",
        "OtherSymbol",
        "Space",
        "LineSeparator",
        "ParagraphSeparator",
        "Control",
        "Format",
        "Surrogate",
        "PrivateUse",
        "NotAssigned",
    ] {
        env_chirho.bind_chirho(
            general_category_ctor_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::ConChirho("GeneralCategory".to_string())),
        );
    }

    // intToDigit :: Int -> Char
    env_chirho.bind_chirho(
        "intToDigit".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ConChirho("Char".to_string()),
        )),
    );

    // ── Floating functions ──
    // Unary: sin, cos, tan, asin, acos, atan, exp, log, sqrt, sinh, cosh,
    // tanh, asinh, acosh, atanh :: Floating a => a -> a
    for (index_chirho, name_chirho) in [
        "sin", "cos", "tan", "asin", "acos", "atan", "exp", "log", "sqrt", "sinh", "cosh", "tanh",
        "asinh", "acosh", "atanh",
    ]
    .into_iter()
    .enumerate()
    {
        let floating_a_chirho = TyVarChirho(3600 + index_chirho as u32);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![floating_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Floating".to_string(),
                    ty_chirho: TyChirho::VarChirho(floating_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(floating_a_chirho),
                    TyChirho::VarChirho(floating_a_chirho),
                ),
            },
        );
    }

    // Binary floating operators/functions: (**), atan2, logBase :: Floating a => a -> a -> a
    for (index_chirho, name_chirho) in ["**", "atan2", "logBase"].into_iter().enumerate() {
        let floating_a_chirho = TyVarChirho(3620 + index_chirho as u32);
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![floating_a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Floating".to_string(),
                    ty_chirho: TyChirho::VarChirho(floating_a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(floating_a_chirho),
                        TyChirho::VarChirho(floating_a_chirho),
                    ],
                    TyChirho::VarChirho(floating_a_chirho),
                ),
            },
        );
    }

    // NOTE: pi is not bound at the top level to avoid shadowing user-defined
    // local "pi" bindings. It is available via $prim_Floating_pi_Double.

    // ── Functor / Applicative / Monad ──
    // fmap :: Functor f => (a -> b) -> f a -> f b  (polymorphic)
    {
        let fmap_f_chirho = TyVarChirho(9004);
        let fmap_a_chirho = TyVarChirho(9040);
        let fmap_b_chirho = TyVarChirho(9041);
        let fmap_scheme_chirho = SchemeChirho {
            vars_chirho: vec![fmap_f_chirho, fmap_a_chirho, fmap_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Functor".to_string(),
                ty_chirho: TyChirho::VarChirho(fmap_f_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::FunChirho(
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::VarChirho(fmap_a_chirho)),
                    Box::new(TyChirho::VarChirho(fmap_b_chirho)),
                    MultChirho::ManyChirho,
                )),
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(fmap_f_chirho)),
                        Box::new(TyChirho::VarChirho(fmap_a_chirho)),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(fmap_f_chirho)),
                        Box::new(TyChirho::VarChirho(fmap_b_chirho)),
                    )),
                    MultChirho::ManyChirho,
                )),
                MultChirho::ManyChirho,
            ),
        };
        env_chirho.bind_chirho("fmap".to_string(), fmap_scheme_chirho.clone());
        env_chirho.bind_chirho("<$>".to_string(), fmap_scheme_chirho);
    }

    // (<$) :: Functor f => a -> f b -> f a
    {
        let functor_f_chirho = TyVarChirho(9042);
        let left_a_chirho = TyVarChirho(9043);
        let right_b_chirho = TyVarChirho(9044);
        let replace_left_scheme_chirho = SchemeChirho {
            vars_chirho: vec![functor_f_chirho, left_a_chirho, right_b_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Functor".to_string(),
                ty_chirho: TyChirho::VarChirho(functor_f_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::FunChirho(
                Box::new(TyChirho::VarChirho(left_a_chirho)),
                Box::new(TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(functor_f_chirho)),
                        Box::new(TyChirho::VarChirho(right_b_chirho)),
                    )),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(functor_f_chirho)),
                        Box::new(TyChirho::VarChirho(left_a_chirho)),
                    )),
                    MultChirho::ManyChirho,
                )),
                MultChirho::ManyChirho,
            ),
        };
        env_chirho.bind_chirho("<$".to_string(), replace_left_scheme_chirho);
        env_chirho.bind_chirho(
            "$>".to_string(),
            SchemeChirho {
                vars_chirho: vec![functor_f_chirho, left_a_chirho, right_b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Functor".to_string(),
                    ty_chirho: TyChirho::VarChirho(functor_f_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(functor_f_chirho)),
                        Box::new(TyChirho::VarChirho(left_a_chirho)),
                    )),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(right_b_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(functor_f_chirho)),
                            Box::new(TyChirho::VarChirho(right_b_chirho)),
                        )),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // (<&>) :: Functor f => f a -> (a -> b) -> f b
    {
        let functor_f_chirho = TyVarChirho(9045);
        let left_a_chirho = TyVarChirho(9046);
        let right_b_chirho = TyVarChirho(9047);
        env_chirho.bind_chirho(
            "<&>".to_string(),
            SchemeChirho {
                vars_chirho: vec![functor_f_chirho, left_a_chirho, right_b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Functor".to_string(),
                    ty_chirho: TyChirho::VarChirho(functor_f_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(functor_f_chirho)),
                        Box::new(TyChirho::VarChirho(left_a_chirho)),
                    )),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::FunChirho(
                            Box::new(TyChirho::VarChirho(left_a_chirho)),
                            Box::new(TyChirho::VarChirho(right_b_chirho)),
                            MultChirho::ManyChirho,
                        )),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(functor_f_chirho)),
                            Box::new(TyChirho::VarChirho(right_b_chirho)),
                        )),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }

    // foldMap :: (a -> m) -> t a -> m  (simplified, Monoid m not enforced)
    {
        let fold_a_chirho = TyVarChirho(9081);
        let fold_m_chirho = TyVarChirho(9082);
        let fold_t_chirho = TyVarChirho(9080);
        env_chirho.bind_chirho(
            "foldMap".to_string(),
            SchemeChirho {
                vars_chirho: vec![fold_t_chirho, fold_a_chirho, fold_m_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::FunChirho(
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::VarChirho(fold_a_chirho)),
                        Box::new(TyChirho::VarChirho(fold_m_chirho)),
                        MultChirho::ManyChirho,
                    )),
                    Box::new(TyChirho::FunChirho(
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(fold_t_chirho)),
                            Box::new(TyChirho::VarChirho(fold_a_chirho)),
                        )),
                        Box::new(TyChirho::VarChirho(fold_m_chirho)),
                        MultChirho::ManyChirho,
                    )),
                    MultChirho::ManyChirho,
                ),
            },
        );
    }
}
