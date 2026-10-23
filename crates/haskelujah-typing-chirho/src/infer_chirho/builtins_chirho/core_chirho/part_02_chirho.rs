// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 02 of 14: original lines 8777-9717 of
//! `seed_builtins_chirho`, in order (55 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_02_chirho(env_chirho: &mut TyEnvChirho) {
    let text_ty_chirho = TyChirho::ConChirho("Text".to_string());
    let byte_string_ty_chirho = TyChirho::ConChirho("ByteString".to_string());
    let case_insensitive_byte_string_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("CI".to_string())),
        Box::new(byte_string_ty_chirho.clone()),
    );
    let char_list_ty_chirho = TyChirho::ListChirho(Box::new(TyChirho::char_chirho()));
    let word8_list_ty_chirho =
        TyChirho::ListChirho(Box::new(TyChirho::ConChirho("Word8".to_string())));
    let maybe_text_uncons_ty_chirho = TyChirho::fun_chirho(
        text_ty_chirho.clone(),
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::TupleChirho(vec![
                TyChirho::char_chirho(),
                text_ty_chirho.clone(),
            ])),
        ),
    );
    let maybe_char_byte_string_uncons_ty_chirho = TyChirho::fun_chirho(
        byte_string_ty_chirho.clone(),
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::TupleChirho(vec![
                TyChirho::char_chirho(),
                byte_string_ty_chirho.clone(),
            ])),
        ),
    );
    let maybe_word8_byte_string_uncons_ty_chirho = TyChirho::fun_chirho(
        byte_string_ty_chirho.clone(),
        TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Maybe".to_string())),
            Box::new(TyChirho::TupleChirho(vec![
                TyChirho::ConChirho("Word8".to_string()),
                byte_string_ty_chirho.clone(),
            ])),
        ),
    );

    for module_name_chirho in ["Data.Text", "Data.Text.Lazy"] {
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.pack"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                char_list_ty_chirho.clone(),
                text_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.unpack"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                text_ty_chirho.clone(),
                char_list_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.uncons"),
            SchemeChirho::mono_chirho(maybe_text_uncons_ty_chirho.clone()),
        );
    }
    env_chirho.bind_chirho(
        "utf8LengthByLeader".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Word8".to_string()),
            TyChirho::int_chirho(),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.Internal.Encoding.Utf8.utf8LengthByLeader".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Word8".to_string()),
            TyChirho::int_chirho(),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.IO.readFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(text_ty_chirho.clone()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.IO.writeFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), text_ty_chirho.clone()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.IO.putStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            text_ty_chirho.clone(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.IO.putStrLn".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            text_ty_chirho.clone(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.IO.getContents".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(text_ty_chirho.clone())),
    );
    let text_lazy_ty_chirho = TyChirho::ConChirho("Text".to_string());

    env_chirho.bind_chirho(
        "Data.Text.Lazy.IO.readFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(text_lazy_ty_chirho.clone()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.Lazy.IO.writeFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), text_lazy_ty_chirho.clone()],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.Lazy.IO.putStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            text_lazy_ty_chirho.clone(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.Lazy.IO.putStrLn".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            text_lazy_ty_chirho.clone(),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );
    env_chirho.bind_chirho(
        "Data.Text.Lazy.IO.getContents".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(text_lazy_ty_chirho.clone())),
    );

    for module_name_chirho in ["Data.ByteString", "Data.ByteString.Lazy"] {
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.pack"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                word8_list_ty_chirho.clone(),
                byte_string_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.unpack"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                byte_string_ty_chirho.clone(),
                word8_list_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.uncons"),
            SchemeChirho::mono_chirho(maybe_word8_byte_string_uncons_ty_chirho.clone()),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.readFile"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::io_chirho(byte_string_ty_chirho.clone()),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.writeFile"),
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![TyChirho::string_chirho(), byte_string_ty_chirho.clone()],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.putStr"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                byte_string_ty_chirho.clone(),
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.putStrLn"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                byte_string_ty_chirho.clone(),
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.getContents"),
            SchemeChirho::mono_chirho(TyChirho::io_chirho(byte_string_ty_chirho.clone())),
        );
    }

    {
        let byte_string_callback_a_chirho = TyVarChirho(1687);
        let c_string_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(TyChirho::ConChirho("CChar".to_string())),
        );
        let c_string_len_ty_chirho =
            TyChirho::TupleChirho(vec![c_string_ty_chirho, TyChirho::int_chirho()]);
        let byte_string_callback_result_ty_chirho =
            TyChirho::io_chirho(TyChirho::VarChirho(byte_string_callback_a_chirho));
        let use_as_cstring_len_scheme_chirho = SchemeChirho {
            vars_chirho: vec![byte_string_callback_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    byte_string_ty_chirho.clone(),
                    TyChirho::fun_chirho(
                        c_string_len_ty_chirho.clone(),
                        byte_string_callback_result_ty_chirho.clone(),
                    ),
                ],
                byte_string_callback_result_ty_chirho,
            ),
        };
        let pack_cstring_len_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            c_string_len_ty_chirho,
            TyChirho::io_chirho(byte_string_ty_chirho.clone()),
        ));
        for name_chirho in [
            "packCStringLen",
            "Data.ByteString.packCStringLen",
            "Data.ByteString.Char8.packCStringLen",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                pack_cstring_len_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["useAsCStringLen", "Data.ByteString.useAsCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                use_as_cstring_len_scheme_chirho.clone(),
            );
        }
    }

    for module_name_chirho in ["Data.ByteString.Char8", "Data.ByteString.Lazy.Char8"] {
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.pack"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                char_list_ty_chirho.clone(),
                byte_string_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.unpack"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                byte_string_ty_chirho.clone(),
                char_list_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.uncons"),
            SchemeChirho::mono_chirho(maybe_char_byte_string_uncons_ty_chirho.clone()),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.readFile"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::io_chirho(byte_string_ty_chirho.clone()),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.writeFile"),
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![TyChirho::string_chirho(), byte_string_ty_chirho.clone()],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            )),
        );
        env_chirho.bind_chirho(
            format!("{module_name_chirho}.getContents"),
            SchemeChirho::mono_chirho(TyChirho::io_chirho(byte_string_ty_chirho.clone())),
        );
    }

    // Data.CaseInsensitive (needed by warp/http-types)
    {
        let ci_a_chirho = TyVarChirho(1688);
        let ci_input_ty_chirho = TyChirho::VarChirho(ci_a_chirho);
        let ci_result_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("CI".to_string())),
            Box::new(ci_input_ty_chirho.clone()),
        );
        let mk_scheme_chirho = SchemeChirho {
            vars_chirho: vec![ci_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                ci_input_ty_chirho.clone(),
                ci_result_ty_chirho.clone(),
            ),
        };
        env_chirho.bind_chirho("mk".to_string(), mk_scheme_chirho.clone());
        env_chirho.bind_chirho("Data.CaseInsensitive.mk".to_string(), mk_scheme_chirho);

        let original_scheme_chirho = SchemeChirho {
            vars_chirho: vec![ci_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(ci_result_ty_chirho, ci_input_ty_chirho),
        };
        env_chirho.bind_chirho("original".to_string(), original_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.CaseInsensitive.original".to_string(),
            original_scheme_chirho,
        );

        let fold_case_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            byte_string_ty_chirho.clone(),
            byte_string_ty_chirho.clone(),
        ));
        env_chirho.bind_chirho("foldCase".to_string(), fold_case_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.CaseInsensitive.foldCase".to_string(),
            fold_case_scheme_chirho,
        );

        let folded_case_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            case_insensitive_byte_string_ty_chirho.clone(),
            byte_string_ty_chirho.clone(),
        ));
        env_chirho.bind_chirho("foldedCase".to_string(), folded_case_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.CaseInsensitive.foldedCase".to_string(),
            folded_case_scheme_chirho,
        );
    }

    // toUpper :: Char -> Char
    env_chirho.bind_chirho(
        "toUpper".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::char_chirho(),
        )),
    );

    // toLower :: Char -> Char
    env_chirho.bind_chirho(
        "toLower".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::char_chirho(),
        )),
    );

    // isDigit :: Char -> Bool
    env_chirho.bind_chirho(
        "isDigit".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // isAlpha :: Char -> Bool
    env_chirho.bind_chirho(
        "isAlpha".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::bool_chirho(),
        )),
    );

    // compare :: forall a. Ord a => a -> a -> Ordering
    let cmp_v_chirho = TyVarChirho(1350);
    env_chirho.bind_chirho(
        "compare".to_string(),
        SchemeChirho {
            vars_chirho: vec![cmp_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Ord".to_string(),
                ty_chirho: TyChirho::VarChirho(cmp_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(cmp_v_chirho),
                    TyChirho::VarChirho(cmp_v_chirho),
                ],
                TyChirho::ConChirho("Ordering".to_string()),
            ),
        },
    );

    // min, max :: forall a. Ord a => a -> a -> a
    let minmax_v_chirho = TyVarChirho(1360);
    let minmax_scheme_chirho = SchemeChirho {
        vars_chirho: vec![minmax_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Ord".to_string(),
            ty_chirho: TyChirho::VarChirho(minmax_v_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(minmax_v_chirho),
                TyChirho::VarChirho(minmax_v_chirho),
            ],
            TyChirho::VarChirho(minmax_v_chirho),
        ),
    };
    env_chirho.bind_chirho("min".to_string(), minmax_scheme_chirho.clone());
    env_chirho.bind_chirho("max".to_string(), minmax_scheme_chirho);

    // show :: forall a. Show a => a -> String
    let show_v_chirho = TyVarChirho(1400);
    env_chirho.bind_chirho(
        "show".to_string(),
        SchemeChirho {
            vars_chirho: vec![show_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(show_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(show_v_chirho),
                TyChirho::string_chirho(),
            ),
        },
    );

    // show combinators from GHC.Show / Prelude
    env_chirho.bind_chirho(
        "showString".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::ConChirho("ShowS".to_string()),
        )),
    );
    env_chirho.bind_chirho(
        "showChar".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::char_chirho(),
            TyChirho::ConChirho("ShowS".to_string()),
        )),
    );
    env_chirho.bind_chirho(
        "showParen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::bool_chirho(),
                TyChirho::ConChirho("ShowS".to_string()),
            ],
            TyChirho::ConChirho("ShowS".to_string()),
        )),
    );
    env_chirho.bind_chirho(
        "shows".to_string(),
        SchemeChirho {
            vars_chirho: vec![show_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(show_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(show_v_chirho),
                TyChirho::ConChirho("ShowS".to_string()),
            ),
        },
    );
    env_chirho.bind_chirho(
        "showsPrec".to_string(),
        SchemeChirho {
            vars_chirho: vec![show_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(show_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), TyChirho::VarChirho(show_v_chirho)],
                TyChirho::ConChirho("ShowS".to_string()),
            ),
        },
    );
    env_chirho.bind_chirho(
        "showList".to_string(),
        SchemeChirho {
            vars_chirho: vec![show_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Show".to_string(),
                ty_chirho: TyChirho::VarChirho(show_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(show_v_chirho))),
                TyChirho::ConChirho("ShowS".to_string()),
            ),
        },
    );

    // negate :: forall a. Num a => a -> a
    let negate_v_chirho = TyVarChirho(1150);
    env_chirho.bind_chirho(
        "negate".to_string(),
        SchemeChirho {
            vars_chirho: vec![negate_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(negate_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(negate_v_chirho),
                TyChirho::VarChirho(negate_v_chirho),
            ),
        },
    );

    // (^) :: (Num a, Integral b) => a -> b -> a
    let pow_a_chirho = TyVarChirho(1191);
    let pow_b_chirho = TyVarChirho(1192);
    env_chirho.bind_chirho(
        "^".to_string(),
        SchemeChirho {
            vars_chirho: vec![pow_a_chirho, pow_b_chirho],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(pow_a_chirho),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(pow_b_chirho),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(pow_a_chirho),
                    TyChirho::VarChirho(pow_b_chirho),
                ],
                TyChirho::VarChirho(pow_a_chirho),
            ),
        },
    );

    // Data.Bits operations
    let bits_v_chirho = TyVarChirho(1193);
    let bits_binop_chirho = SchemeChirho {
        vars_chirho: vec![bits_v_chirho],
        preds_chirho: vec![SchemePredChirho {
            class_name_chirho: "Bits".to_string(),
            ty_chirho: TyChirho::VarChirho(bits_v_chirho),
            extra_tys_chirho: vec![],
        }],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                TyChirho::VarChirho(bits_v_chirho),
                TyChirho::VarChirho(bits_v_chirho),
            ],
            TyChirho::VarChirho(bits_v_chirho),
        ),
    };
    env_chirho.bind_chirho(".&.".to_string(), bits_binop_chirho.clone());
    env_chirho.bind_chirho(".|.".to_string(), bits_binop_chirho.clone());
    env_chirho.bind_chirho("xor".to_string(), bits_binop_chirho.clone());
    env_chirho.bind_chirho(
        "complement".to_string(),
        SchemeChirho {
            vars_chirho: vec![bits_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bits".to_string(),
                ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(bits_v_chirho),
                TyChirho::VarChirho(bits_v_chirho),
            ),
        },
    );
    for shift_name_chirho in ["shift", "shiftL", "shiftR", "unsafeShiftL", "unsafeShiftR"] {
        env_chirho.bind_chirho(
            shift_name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![bits_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Bits".to_string(),
                    ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(bits_v_chirho), TyChirho::int_chirho()],
                    TyChirho::VarChirho(bits_v_chirho),
                ),
            },
        );
    }
    env_chirho.bind_chirho(
        "bit".to_string(),
        SchemeChirho {
            vars_chirho: vec![bits_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bits".to_string(),
                ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                TyChirho::VarChirho(bits_v_chirho),
            ),
        },
    );
    for update_bit_name_chirho in ["setBit", "clearBit"] {
        env_chirho.bind_chirho(
            update_bit_name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![bits_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Bits".to_string(),
                    ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![TyChirho::VarChirho(bits_v_chirho), TyChirho::int_chirho()],
                    TyChirho::VarChirho(bits_v_chirho),
                ),
            },
        );
    }
    env_chirho.bind_chirho(
        "testBit".to_string(),
        SchemeChirho {
            vars_chirho: vec![bits_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bits".to_string(),
                ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::VarChirho(bits_v_chirho), TyChirho::int_chirho()],
                TyChirho::bool_chirho(),
            ),
        },
    );
    for int_result_bits_name_chirho in ["bitSize", "popCount", "popCountDefault"] {
        env_chirho.bind_chirho(
            int_result_bits_name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![bits_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Bits".to_string(),
                    ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(bits_v_chirho),
                    TyChirho::int_chirho(),
                ),
            },
        );
    }
    env_chirho.bind_chirho(
        "isSigned".to_string(),
        SchemeChirho {
            vars_chirho: vec![bits_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bits".to_string(),
                ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(bits_v_chirho),
                TyChirho::bool_chirho(),
            ),
        },
    );
    env_chirho.bind_chirho(
        "zeroBits".to_string(),
        SchemeChirho {
            vars_chirho: vec![bits_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Bits".to_string(),
                ty_chirho: TyChirho::VarChirho(bits_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::VarChirho(bits_v_chirho),
        },
    );
    let finite_bits_v_chirho = TyVarChirho(1194);
    for finite_bits_name_chirho in ["finiteBitSize", "countLeadingZeros", "countTrailingZeros"] {
        env_chirho.bind_chirho(
            finite_bits_name_chirho.to_string(),
            SchemeChirho {
                vars_chirho: vec![finite_bits_v_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "FiniteBits".to_string(),
                    ty_chirho: TyChirho::VarChirho(finite_bits_v_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(finite_bits_v_chirho),
                    TyChirho::int_chirho(),
                ),
            },
        );
    }
    for (byteswap_name_chirho, word_type_name_chirho) in [
        ("byteSwap16", "Word16"),
        ("byteSwap32", "Word32"),
        ("byteSwap64", "Word64"),
    ] {
        let byteswap_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho(word_type_name_chirho.to_string()),
            TyChirho::ConChirho(word_type_name_chirho.to_string()),
        ));
        for qualified_name_chirho in [
            byteswap_name_chirho.to_string(),
            format!("Data.Word.{byteswap_name_chirho}"),
            format!("GHC.Word.{byteswap_name_chirho}"),
        ] {
            env_chirho.bind_chirho(qualified_name_chirho, byteswap_scheme_chirho.clone());
        }
    }

    // fromInteger :: forall a. Num a => Integer -> a
    let fi_v_chirho = TyVarChirho(1160);
    env_chirho.bind_chirho(
        "fromInteger".to_string(),
        SchemeChirho {
            vars_chirho: vec![fi_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Num".to_string(),
                ty_chirho: TyChirho::VarChirho(fi_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("Integer".to_string()),
                TyChirho::VarChirho(fi_v_chirho),
            ),
        },
    );

    // (/) :: forall a. Fractional a => a -> a -> a
    let div_v_chirho = TyVarChirho(1170);
    env_chirho.bind_chirho(
        "/".to_string(),
        SchemeChirho {
            vars_chirho: vec![div_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Fractional".to_string(),
                ty_chirho: TyChirho::VarChirho(div_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(div_v_chirho),
                    TyChirho::VarChirho(div_v_chirho),
                ],
                TyChirho::VarChirho(div_v_chirho),
            ),
        },
    );

    // recip :: forall a. Fractional a => a -> a
    let recip_v_chirho = TyVarChirho(1180);
    env_chirho.bind_chirho(
        "recip".to_string(),
        SchemeChirho {
            vars_chirho: vec![recip_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Fractional".to_string(),
                ty_chirho: TyChirho::VarChirho(recip_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(recip_v_chirho),
                TyChirho::VarChirho(recip_v_chirho),
            ),
        },
    );

    // fromRational :: forall a. Fractional a => Rational -> a
    let fr_v_chirho = TyVarChirho(1190);
    env_chirho.bind_chirho(
        "fromRational".to_string(),
        SchemeChirho {
            vars_chirho: vec![fr_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Fractional".to_string(),
                ty_chirho: TyChirho::VarChirho(fr_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("Rational".to_string()),
                TyChirho::VarChirho(fr_v_chirho),
            ),
        },
    );

    // (^) :: forall a b. (Num a, Integral b) => a -> b -> a
    let pow_a_chirho = TyVarChirho(1191);
    let pow_b_chirho = TyVarChirho(1192);
    env_chirho.bind_chirho(
        "^".to_string(),
        SchemeChirho {
            vars_chirho: vec![pow_a_chirho, pow_b_chirho],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Num".to_string(),
                    ty_chirho: TyChirho::VarChirho(pow_a_chirho),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(pow_b_chirho),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(pow_a_chirho),
                    TyChirho::VarChirho(pow_b_chirho),
                ],
                TyChirho::VarChirho(pow_a_chirho),
            ),
        },
    );

    // (^^) :: forall a b. (Fractional a, Integral b) => a -> b -> a
    let fpow_a_chirho = TyVarChirho(1193);
    let fpow_b_chirho = TyVarChirho(1194);
    env_chirho.bind_chirho(
        "^^".to_string(),
        SchemeChirho {
            vars_chirho: vec![fpow_a_chirho, fpow_b_chirho],
            preds_chirho: vec![
                SchemePredChirho {
                    class_name_chirho: "Fractional".to_string(),
                    ty_chirho: TyChirho::VarChirho(fpow_a_chirho),
                    extra_tys_chirho: vec![],
                },
                SchemePredChirho {
                    class_name_chirho: "Integral".to_string(),
                    ty_chirho: TyChirho::VarChirho(fpow_b_chirho),
                    extra_tys_chirho: vec![],
                },
            ],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(fpow_a_chirho),
                    TyChirho::VarChirho(fpow_b_chirho),
                ],
                TyChirho::VarChirho(fpow_a_chirho),
            ),
        },
    );

    // read :: forall a. Read a => String -> a
    let read_v_chirho = TyVarChirho(1195);
    env_chirho.bind_chirho(
        "read".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Read".to_string(),
                ty_chirho: TyChirho::VarChirho(read_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::VarChirho(read_v_chirho),
            ),
        },
    );
    env_chirho.bind_chirho(
        "reads".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Read".to_string(),
                ty_chirho: TyChirho::VarChirho(read_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(read_v_chirho),
                    TyChirho::string_chirho(),
                ]))),
            ),
        },
    );
    env_chirho.bind_chirho(
        "readsPrec".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Read".to_string(),
                ty_chirho: TyChirho::VarChirho(read_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![TyChirho::int_chirho(), TyChirho::string_chirho()],
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::VarChirho(read_v_chirho),
                    TyChirho::string_chirho(),
                ]))),
            ),
        },
    );
    env_chirho.bind_chirho(
        "readList".to_string(),
        SchemeChirho {
            vars_chirho: vec![read_v_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "Read".to_string(),
                ty_chirho: TyChirho::VarChirho(read_v_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::ListChirho(Box::new(TyChirho::TupleChirho(vec![
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(read_v_chirho))),
                    TyChirho::string_chirho(),
                ]))),
            ),
        },
    );
    let safe_read_a_chirho = TyVarChirho(71350);
    env_chirho.bind_chirho(
        "Test.Tasty.Options.safeRead".to_string(),
        SchemeChirho {
            vars_chirho: vec![safe_read_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(safe_read_a_chirho)),
                ),
            ),
        },
    );
}
