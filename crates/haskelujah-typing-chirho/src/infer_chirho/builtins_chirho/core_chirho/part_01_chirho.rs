// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 01 of 14: original lines 7869-8775 of
//! `seed_builtins_chirho`, in order (49 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_01_chirho(env_chirho: &mut TyEnvChirho) {
    let th_name_ty_chirho = TyChirho::ConChirho("Name".to_string());
    let th_type_ty_chirho = TyChirho::ConChirho("Type".to_string());
    let th_kind_ty_chirho = TyChirho::ConChirho("Kind".to_string());
    let th_pred_ty_chirho = TyChirho::ConChirho("Pred".to_string());
    let th_cxt_ty_chirho = TyChirho::ConChirho("Cxt".to_string());
    let th_tyvar_bndr_unit_ty_chirho = TyChirho::ConChirho("TyVarBndrUnit".to_string());
    let th_tyvar_bndr_spec_ty_chirho = TyChirho::ConChirho("TyVarBndrSpec".to_string());
    let th_tyvar_bndr_vis_ty_chirho = TyChirho::ConChirho("TyVarBndrVis".to_string());
    let th_specificity_ty_chirho = TyChirho::ConChirho("Specificity".to_string());
    let th_q_tycon_ty_chirho = TyChirho::ConChirho("Q".to_string());

    env_chirho.bind_chirho(
        "mkName".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            th_name_ty_chirho.clone(),
        )),
    );
    env_chirho.bind_chirho(
        "nameBase".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            th_name_ty_chirho.clone(),
            TyChirho::string_chirho(),
        )),
    );
    env_chirho.bind_chirho(
        "conT".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            th_name_ty_chirho.clone(),
            TyChirho::AppChirho(
                Box::new(th_q_tycon_ty_chirho.clone()),
                Box::new(th_type_ty_chirho.clone()),
            ),
        )),
    );
    env_chirho.bind_chirho(
        "varT".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            th_name_ty_chirho.clone(),
            TyChirho::AppChirho(
                Box::new(th_q_tycon_ty_chirho.clone()),
                Box::new(th_type_ty_chirho.clone()),
            ),
        )),
    );
    env_chirho.bind_chirho(
        "appT".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::AppChirho(
                    Box::new(th_q_tycon_ty_chirho.clone()),
                    Box::new(th_type_ty_chirho.clone()),
                ),
                TyChirho::AppChirho(
                    Box::new(th_q_tycon_ty_chirho.clone()),
                    Box::new(th_type_ty_chirho.clone()),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(th_q_tycon_ty_chirho.clone()),
                Box::new(th_type_ty_chirho.clone()),
            ),
        )),
    );
    env_chirho.bind_chirho(
        "appKindT".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::AppChirho(
                    Box::new(th_q_tycon_ty_chirho.clone()),
                    Box::new(th_type_ty_chirho.clone()),
                ),
                TyChirho::AppChirho(
                    Box::new(th_q_tycon_ty_chirho.clone()),
                    Box::new(th_type_ty_chirho.clone()),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(th_q_tycon_ty_chirho.clone()),
                Box::new(th_type_ty_chirho.clone()),
            ),
        )),
    );
    env_chirho.bind_chirho(
        "sigP".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::AppChirho(
                    Box::new(th_q_tycon_ty_chirho.clone()),
                    Box::new(TyChirho::ConChirho("Pat".to_string())),
                ),
                TyChirho::AppChirho(
                    Box::new(th_q_tycon_ty_chirho.clone()),
                    Box::new(th_type_ty_chirho.clone()),
                ),
            ],
            TyChirho::AppChirho(
                Box::new(th_q_tycon_ty_chirho.clone()),
                Box::new(TyChirho::ConChirho("Pat".to_string())),
            ),
        )),
    );
    for (name_chirho, ty_chirho) in [
        (
            "ConT",
            TyChirho::fun_chirho(th_name_ty_chirho.clone(), th_type_ty_chirho.clone()),
        ),
        (
            "VarT",
            TyChirho::fun_chirho(th_name_ty_chirho.clone(), th_type_ty_chirho.clone()),
        ),
        (
            "AppT",
            TyChirho::fun_n_chirho(
                vec![th_type_ty_chirho.clone(), th_type_ty_chirho.clone()],
                th_type_ty_chirho.clone(),
            ),
        ),
        (
            "AppKindT",
            TyChirho::fun_n_chirho(
                vec![th_type_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_type_ty_chirho.clone(),
            ),
        ),
        (
            "SigT",
            TyChirho::fun_n_chirho(
                vec![th_type_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_type_ty_chirho.clone(),
            ),
        ),
        (
            "ForallT",
            TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(th_tyvar_bndr_spec_ty_chirho.clone())),
                    th_cxt_ty_chirho.clone(),
                    th_type_ty_chirho.clone(),
                ],
                th_type_ty_chirho.clone(),
            ),
        ),
        (
            "InfixT",
            TyChirho::fun_n_chirho(
                vec![
                    th_type_ty_chirho.clone(),
                    th_name_ty_chirho.clone(),
                    th_type_ty_chirho.clone(),
                ],
                th_type_ty_chirho.clone(),
            ),
        ),
        (
            "ParensT",
            TyChirho::fun_chirho(th_type_ty_chirho.clone(), th_type_ty_chirho.clone()),
        ),
        ("ArrowT", th_type_ty_chirho.clone()),
        ("ListT", th_type_ty_chirho.clone()),
        ("StarT", th_type_ty_chirho.clone()),
        (
            "TupleT",
            TyChirho::fun_chirho(TyChirho::int_chirho(), th_type_ty_chirho.clone()),
        ),
        (
            "UnboxedTupleT",
            TyChirho::fun_chirho(TyChirho::int_chirho(), th_type_ty_chirho.clone()),
        ),
        (
            "PlainTV",
            TyChirho::fun_chirho(
                th_name_ty_chirho.clone(),
                th_tyvar_bndr_unit_ty_chirho.clone(),
            ),
        ),
        (
            "KindedTV",
            TyChirho::fun_n_chirho(
                vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_tyvar_bndr_unit_ty_chirho.clone(),
            ),
        ),
        (
            "classPred",
            TyChirho::fun_n_chirho(
                vec![
                    th_name_ty_chirho.clone(),
                    TyChirho::ListChirho(Box::new(th_type_ty_chirho.clone())),
                ],
                th_pred_ty_chirho.clone(),
            ),
        ),
        (
            "appK",
            TyChirho::fun_n_chirho(
                vec![th_kind_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_kind_ty_chirho.clone(),
            ),
        ),
        ("arrowK", th_kind_ty_chirho.clone()),
        ("starK", th_kind_ty_chirho.clone()),
        ("SpecifiedSpec", th_specificity_ty_chirho.clone()),
        ("InferredSpec", th_specificity_ty_chirho.clone()),
        (
            "plainTV",
            TyChirho::fun_chirho(
                th_name_ty_chirho.clone(),
                th_tyvar_bndr_unit_ty_chirho.clone(),
            ),
        ),
        (
            "kindedTV",
            TyChirho::fun_n_chirho(
                vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_tyvar_bndr_unit_ty_chirho.clone(),
            ),
        ),
        (
            "plainTVSpecified",
            TyChirho::fun_chirho(
                th_name_ty_chirho.clone(),
                th_tyvar_bndr_spec_ty_chirho.clone(),
            ),
        ),
        (
            "plainTVInferred",
            TyChirho::fun_chirho(
                th_name_ty_chirho.clone(),
                th_tyvar_bndr_spec_ty_chirho.clone(),
            ),
        ),
        (
            "kindedTVSpecified",
            TyChirho::fun_n_chirho(
                vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_tyvar_bndr_spec_ty_chirho.clone(),
            ),
        ),
        (
            "kindedTVInferred",
            TyChirho::fun_n_chirho(
                vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_tyvar_bndr_spec_ty_chirho.clone(),
            ),
        ),
        (
            "kindedTVReq",
            TyChirho::fun_n_chirho(
                vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                th_tyvar_bndr_vis_ty_chirho.clone(),
            ),
        ),
        (
            "elimTV",
            TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(th_name_ty_chirho.clone(), th_name_ty_chirho.clone()),
                    TyChirho::fun_n_chirho(
                        vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                        th_name_ty_chirho.clone(),
                    ),
                    th_tyvar_bndr_unit_ty_chirho.clone(),
                ],
                th_name_ty_chirho.clone(),
            ),
        ),
        (
            "tvName",
            TyChirho::fun_chirho(
                th_tyvar_bndr_unit_ty_chirho.clone(),
                th_name_ty_chirho.clone(),
            ),
        ),
        (
            "tvKind",
            TyChirho::fun_chirho(
                th_tyvar_bndr_unit_ty_chirho.clone(),
                th_kind_ty_chirho.clone(),
            ),
        ),
    ] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            SchemeChirho::mono_chirho(ty_chirho),
        );
    }
    let forall_tvb_chirho = TyVarChirho(6316);
    env_chirho.bind_chirho(
        "ForallT".to_string(),
        SchemeChirho {
            vars_chirho: vec![forall_tvb_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(forall_tvb_chirho))),
                    th_cxt_ty_chirho.clone(),
                    th_type_ty_chirho.clone(),
                ],
                th_type_ty_chirho.clone(),
            ),
        },
    );
    let plain_result_chirho = TyVarChirho(6317);
    env_chirho.bind_chirho(
        "PlainTV".to_string(),
        SchemeChirho {
            vars_chirho: vec![plain_result_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                th_name_ty_chirho.clone(),
                TyChirho::VarChirho(plain_result_chirho),
            ),
        },
    );
    let kinded_flag_chirho = TyVarChirho(6318);
    let kinded_result_chirho = TyVarChirho(6319);
    env_chirho.bind_chirho(
        "KindedTV".to_string(),
        SchemeChirho {
            vars_chirho: vec![kinded_flag_chirho, kinded_result_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    th_name_ty_chirho.clone(),
                    TyChirho::VarChirho(kinded_flag_chirho),
                ],
                TyChirho::VarChirho(kinded_result_chirho),
            ),
        },
    );
    let elim_tvb_chirho = TyVarChirho(6320);
    let elim_result_chirho = TyVarChirho(6321);
    env_chirho.bind_chirho(
        "elimTV".to_string(),
        SchemeChirho {
            vars_chirho: vec![elim_tvb_chirho, elim_result_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::fun_chirho(
                        th_name_ty_chirho.clone(),
                        TyChirho::VarChirho(elim_result_chirho),
                    ),
                    TyChirho::fun_n_chirho(
                        vec![th_name_ty_chirho.clone(), th_kind_ty_chirho.clone()],
                        TyChirho::VarChirho(elim_result_chirho),
                    ),
                    TyChirho::VarChirho(elim_tvb_chirho),
                ],
                TyChirho::VarChirho(elim_result_chirho),
            ),
        },
    );
    let tv_name_tvb_chirho = TyVarChirho(6322);
    env_chirho.bind_chirho(
        "tvName".to_string(),
        SchemeChirho {
            vars_chirho: vec![tv_name_tvb_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(tv_name_tvb_chirho),
                th_name_ty_chirho.clone(),
            ),
        },
    );
    let tv_kind_tvb_chirho = TyVarChirho(6323);
    env_chirho.bind_chirho(
        "tvKind".to_string(),
        SchemeChirho {
            vars_chirho: vec![tv_kind_tvb_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(tv_kind_tvb_chirho),
                th_kind_ty_chirho.clone(),
            ),
        },
    );

    // not :: Bool -> Bool
    env_chirho.bind_chirho(
        "not".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::bool_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // (&&) :: Bool -> Bool -> Bool
    env_chirho.bind_chirho(
        "&&".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::bool_chirho(), TyChirho::bool_chirho()],
            TyChirho::bool_chirho(),
        )),
    );

    // (||) :: Bool -> Bool -> Bool
    env_chirho.bind_chirho(
        "||".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            [TyChirho::bool_chirho(), TyChirho::bool_chirho()],
            TyChirho::bool_chirho(),
        )),
    );

    // (++) :: forall a. [a] -> [a] -> [a]
    let append_v_chirho = TyVarChirho(1099);
    env_chirho.bind_chirho(
        "++".to_string(),
        SchemeChirho {
            vars_chirho: vec![append_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_v_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_v_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(append_v_chirho))),
            ),
        },
    );

    // Numeric operators: forall a. Num a => a -> a -> a
    let num_v_chirho = TyVarChirho(1100);
    let num_binop_chirho = SchemeChirho {
        vars_chirho: vec![num_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Num".to_string(),
            ty_chirho: TyChirho::VarChirho(num_v_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(num_v_chirho),
                TyChirho::VarChirho(num_v_chirho),
            ],
            TyChirho::VarChirho(num_v_chirho),
        ),
    };
    env_chirho.bind_chirho("+".to_string(), num_binop_chirho.clone());
    env_chirho.bind_chirho("-".to_string(), num_binop_chirho.clone());
    env_chirho.bind_chirho("*".to_string(), num_binop_chirho);

    // Equality operators: forall a. Eq a => a -> a -> Bool
    let eq_v_chirho = TyVarChirho(1200);
    let eq_cmp_chirho = SchemeChirho {
        vars_chirho: vec![eq_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Eq".to_string(),
            ty_chirho: TyChirho::VarChirho(eq_v_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(eq_v_chirho),
                TyChirho::VarChirho(eq_v_chirho),
            ],
            TyChirho::bool_chirho(),
        ),
    };
    env_chirho.bind_chirho("==".to_string(), eq_cmp_chirho.clone());
    env_chirho.bind_chirho("/=".to_string(), eq_cmp_chirho);

    // Ordering operators: forall a. Ord a => a -> a -> Bool
    let ord_v_chirho = TyVarChirho(1300);
    let ord_cmp_chirho = SchemeChirho {
        vars_chirho: vec![ord_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Ord".to_string(),
            ty_chirho: TyChirho::VarChirho(ord_v_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(ord_v_chirho),
                TyChirho::VarChirho(ord_v_chirho),
            ],
            TyChirho::bool_chirho(),
        ),
    };
    env_chirho.bind_chirho(">".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho("<".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho(">=".to_string(), ord_cmp_chirho.clone());
    env_chirho.bind_chirho("<=".to_string(), ord_cmp_chirho);

    // ord :: Char -> Int
    env_chirho.bind_chirho(
        "ord".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::int_chirho(),
        )),
    );

    // chr :: Int -> Char
    env_chirho.bind_chirho(
        "chr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::char_chirho(),
        )),
    );

    let time_locale_ty_chirho = TyChirho::ConChirho("TimeLocale".to_string());
    env_chirho.bind_chirho(
        "defaultTimeLocale".to_string(),
        SchemeChirho::mono_chirho(time_locale_ty_chirho.clone()),
    );

    let format_time_t_chirho = TyVarChirho(1687);
    env_chirho.bind_chirho(
        "formatTime".to_string(),
        SchemeChirho {
            vars_chirho: vec![format_time_t_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "FormatTime".to_string(),
                ty_chirho: TyChirho::VarChirho(format_time_t_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    time_locale_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                    TyChirho::VarChirho(format_time_t_chirho),
                ],
                TyChirho::string_chirho(),
            ),
        },
    );

    let parse_time_m_chirho = TyVarChirho(1688);
    let parse_time_t_chirho = TyVarChirho(1689);
    env_chirho.bind_chirho(
        "parseTimeM".to_string(),
        SchemeChirho {
            vars_chirho: vec![parse_time_m_chirho, parse_time_t_chirho],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "MonadFail".to_string(),
                    ty_chirho: TyChirho::VarChirho(parse_time_m_chirho),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "ParseTime".to_string(),
                    ty_chirho: TyChirho::VarChirho(parse_time_t_chirho),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::bool_chirho(),
                    time_locale_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                    TyChirho::string_chirho(),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(parse_time_m_chirho)),
                    Box::new(TyChirho::VarChirho(parse_time_t_chirho)),
                ),
            ),
        },
    );

    let parse_time_or_error_t_chirho = TyVarChirho(1690);
    env_chirho.bind_chirho(
        "parseTimeOrError".to_string(),
        SchemeChirho {
            vars_chirho: vec![parse_time_or_error_t_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "ParseTime".to_string(),
                ty_chirho: TyChirho::VarChirho(parse_time_or_error_t_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::bool_chirho(),
                    time_locale_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                    TyChirho::string_chirho(),
                ],
                TyChirho::VarChirho(parse_time_or_error_t_chirho),
            ),
        },
    );

    let read_s_time_t_chirho = TyVarChirho(1691);
    env_chirho.bind_chirho(
        "readSTime".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_s_time_t_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "ParseTime".to_string(),
                ty_chirho: TyChirho::VarChirho(read_s_time_t_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::bool_chirho(),
                    time_locale_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ReadS".to_string())),
                    Box::new(TyChirho::VarChirho(read_s_time_t_chirho)),
                ),
            ),
        },
    );

    let read_p_time_t_chirho = TyVarChirho(1692);
    env_chirho.bind_chirho(
        "readPTime".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_p_time_t_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "ParseTime".to_string(),
                ty_chirho: TyChirho::VarChirho(read_p_time_t_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::bool_chirho(),
                    time_locale_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                ],
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("ReadP".to_string())),
                    Box::new(TyChirho::VarChirho(read_p_time_t_chirho)),
                ),
            ),
        },
    );

    let mk_parsec_gen_parser_ty_chirho =
        |token_ty_chirho: TyChirho, state_ty_chirho: TyChirho, result_ty_chirho: TyChirho| {
            TyChirho::AppChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("GenParser".to_string())),
                        Box::new(token_ty_chirho),
                    )),
                    Box::new(state_ty_chirho),
                )),
                Box::new(result_ty_chirho),
            )
        };
    let mk_parsec_either_ty_chirho = |left_ty_chirho: TyChirho, right_ty_chirho: TyChirho| {
        TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("Either".to_string())),
                Box::new(left_ty_chirho),
            )),
            Box::new(right_ty_chirho),
        )
    };
    let parsec_tok_chirho = TyVarChirho(1693);
    let parsec_state_chirho = TyVarChirho(1694);
    let parsec_a_chirho = TyVarChirho(1695);
    let parsec_parser_a_ty_chirho = mk_parsec_gen_parser_ty_chirho(
        TyChirho::VarChirho(parsec_tok_chirho),
        TyChirho::VarChirho(parsec_state_chirho),
        TyChirho::VarChirho(parsec_a_chirho),
    );
    let parsec_error_ty_chirho = TyChirho::ConChirho("ParseError".to_string());
    let mut bind_parsec_value_chirho = |name_chirho: &str, scheme_chirho: SchemeChirho| {
        env_chirho.bind_chirho(name_chirho.to_string(), scheme_chirho.clone());
        env_chirho.bind_chirho(
            format!("Text.ParserCombinators.Parsec.{name_chirho}"),
            scheme_chirho,
        );
    };
    for name_chirho in ["try", "lookAhead"] {
        bind_parsec_value_chirho(
            name_chirho,
            SchemeChirho {
                vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    parsec_parser_a_ty_chirho.clone(),
                    parsec_parser_a_ty_chirho.clone(),
                ),
            },
        );
    }
    bind_parsec_value_chirho(
        "(<?>)",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![parsec_parser_a_ty_chirho.clone(), TyChirho::string_chirho()],
                parsec_parser_a_ty_chirho.clone(),
            ),
        },
    );
    bind_parsec_value_chirho(
        "option",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(parsec_a_chirho),
                    parsec_parser_a_ty_chirho.clone(),
                ],
                parsec_parser_a_ty_chirho.clone(),
            ),
        },
    );
    bind_parsec_value_chirho(
        "many1",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                parsec_parser_a_ty_chirho.clone(),
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::VarChirho(parsec_tok_chirho),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(parsec_a_chirho))),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "count",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), parsec_parser_a_ty_chirho.clone()],
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::VarChirho(parsec_tok_chirho),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(parsec_a_chirho))),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "notFollowedBy",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                parsec_parser_a_ty_chirho.clone(),
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::VarChirho(parsec_tok_chirho),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::unit_chirho(),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "satisfy",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::fun_chirho(
                    TyChirho::VarChirho(parsec_tok_chirho),
                    TyChirho::bool_chirho(),
                ),
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::VarChirho(parsec_tok_chirho),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::VarChirho(parsec_tok_chirho),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "char",
        SchemeChirho {
            vars_chirho: vec![parsec_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::char_chirho(),
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::char_chirho(),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::char_chirho(),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "oneOf",
        SchemeChirho {
            vars_chirho: vec![parsec_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::char_chirho(),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::char_chirho(),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "string",
        SchemeChirho {
            vars_chirho: vec![parsec_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                mk_parsec_gen_parser_ty_chirho(
                    TyChirho::char_chirho(),
                    TyChirho::VarChirho(parsec_state_chirho),
                    TyChirho::string_chirho(),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "eof",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho],
            preds_chirho: vec![],
            ty_chirho: mk_parsec_gen_parser_ty_chirho(
                TyChirho::VarChirho(parsec_tok_chirho),
                TyChirho::VarChirho(parsec_state_chirho),
                TyChirho::unit_chirho(),
            ),
        },
    );
    bind_parsec_value_chirho(
        "unexpected",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                parsec_parser_a_ty_chirho.clone(),
            ),
        },
    );
    bind_parsec_value_chirho(
        "parse",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    parsec_parser_a_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(parsec_tok_chirho))),
                ],
                mk_parsec_either_ty_chirho(
                    parsec_error_ty_chirho.clone(),
                    TyChirho::VarChirho(parsec_a_chirho),
                ),
            ),
        },
    );
    bind_parsec_value_chirho(
        "parseTest",
        SchemeChirho {
            vars_chirho: vec![parsec_tok_chirho, parsec_state_chirho, parsec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    parsec_parser_a_ty_chirho.clone(),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(parsec_tok_chirho))),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        },
    );

    let assert_a_chirho = TyVarChirho(1686);
    let assert_scheme_chirho = SchemeChirho {
        vars_chirho: vec![assert_a_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::bool_chirho(),
                TyChirho::VarChirho(assert_a_chirho),
            ],
            TyChirho::VarChirho(assert_a_chirho),
        ),
    };
    for name_chirho in ["assert", "GHC.Base.assert", "Control.Exception.assert"] {
        env_chirho.bind_chirho(name_chirho.to_string(), assert_scheme_chirho.clone());
    }

    // unpack :: String -> [Char] (convert C string to cons-list)
    env_chirho.bind_chirho(
        "unpack".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ListChirho(Box::new(TyChirho::char_chirho())),
        )),
    );

    // pack :: [Char] -> String (convert cons-list to C string)
    env_chirho.bind_chirho(
        "pack".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ListChirho(Box::new(TyChirho::char_chirho())),
            TyChirho::string_chirho(),
        )),
    );
}
