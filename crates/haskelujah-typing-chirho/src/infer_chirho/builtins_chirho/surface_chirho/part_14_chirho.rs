// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 14 of 14: original lines 20141-21009 of
//! `seed_builtins_chirho`, in order (43 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_14_chirho(env_chirho: &mut TyEnvChirho) {
    // (!!) :: forall a. [a] -> Int -> a
    {
        let a_chirho = TyVarChirho(7504);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "!!".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho, TyChirho::int_chirho()],
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // isInfixOf :: forall a. Eq a => [a] -> [a] -> Bool
    {
        let a_chirho = TyVarChirho(7503);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "isInfixOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho.clone(), list_a_chirho],
                    TyChirho::bool_chirho(),
                ),
            },
        );
    }

    // stripPrefix :: forall a. Eq a => [a] -> [a] -> Maybe [a]
    {
        let a_chirho = TyVarChirho(7504);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "stripPrefix".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![list_a_chirho.clone(), list_a_chirho.clone()],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(list_a_chirho),
                    ),
                ),
            },
        );
    }

    // findIndex :: forall a. (a -> Bool) -> [a] -> Maybe Int
    {
        let a_chirho = TyVarChirho(7505);
        env_chirho.bind_chirho(
            "findIndex".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::bool_chirho(),
                        ),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::int_chirho()),
                    ),
                ),
            },
        );
    }

    // elemIndex :: forall a. Eq a => a -> [a] -> Maybe Int
    {
        let a_chirho = TyVarChirho(7506);
        env_chirho.bind_chirho(
            "elemIndex".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Eq".to_string(),
                    ty_chirho: TyChirho::VarChirho(a_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::int_chirho()),
                    ),
                ),
            },
        );
    }

    // scanl1 :: forall a. (a -> a -> a) -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7507);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "scanl1".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                            TyChirho::VarChirho(a_chirho),
                        ),
                        list_a_chirho.clone(),
                    ],
                    list_a_chirho,
                ),
            },
        );
    }

    // foldl1' :: forall a. (a -> a -> a) -> [a] -> a
    {
        let a_chirho = TyVarChirho(7507_1);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "foldl1'".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                            TyChirho::VarChirho(a_chirho),
                        ),
                        list_a_chirho,
                    ],
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // scanr :: forall a b. (a -> b -> b) -> b -> [a] -> [b]
    {
        let a_chirho = TyVarChirho(7508);
        let b_chirho = TyVarChirho(7509);
        env_chirho.bind_chirho(
            "scanr".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::VarChirho(b_chirho),
                        ),
                        TyChirho::VarChirho(b_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho))),
                ),
            },
        );
    }

    // scanr1 :: forall a. (a -> a -> a) -> [a] -> [a]
    {
        let a_chirho = TyVarChirho(7510);
        let list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho)));
        env_chirho.bind_chirho(
            "scanr1".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(a_chirho)],
                            TyChirho::VarChirho(a_chirho),
                        ),
                        list_a_chirho.clone(),
                    ],
                    list_a_chirho,
                ),
            },
        );
    }

    // unfoldr :: forall a b. (b -> Maybe (a, b)) -> b -> [a]
    {
        let a_chirho = TyVarChirho(7511);
        let b_chirho = TyVarChirho(7512);
        env_chirho.bind_chirho(
            "unfoldr".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(b_chirho),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::ConChirho("Maybe".to_string())),
                                Box::new(TyChirho::TupleChirho(vec![
                                    TyChirho::VarChirho(a_chirho),
                                    TyChirho::VarChirho(b_chirho),
                                ])),
                            ),
                        ),
                        TyChirho::VarChirho(b_chirho),
                    ],
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_chirho))),
                ),
            },
        );
    }

    // transpose :: forall a. [[a]] -> [[a]]
    {
        let a_chirho = TyVarChirho(7513);
        let list_list_a_chirho = TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
            TyChirho::VarChirho(a_chirho),
        ))));
        env_chirho.bind_chirho(
            "transpose".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(list_list_a_chirho.clone(), list_list_a_chirho),
            },
        );
    }

    // stdin, stdout, stderr :: Handle
    for handle_name_chirho in &["stdin", "stdout", "stderr"] {
        env_chirho.bind_chirho(
            handle_name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::ConChirho("Handle".to_string())),
        );
    }

    // hFlush :: Handle -> IO ()
    env_chirho.bind_chirho(
        "hFlush".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hSetBuffering :: Handle -> BufferMode -> IO ()
    env_chirho.bind_chirho(
        "hSetBuffering".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Handle".to_string()),
                TyChirho::ConChirho("BufferMode".to_string()),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hGetBuffering :: Handle -> IO BufferMode
    env_chirho.bind_chirho(
        "hGetBuffering".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::ConChirho("BufferMode".to_string())),
        )),
    );

    // hIsTerminalDevice :: Handle -> IO Bool
    env_chirho.bind_chirho(
        "hIsTerminalDevice".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::bool_chirho()),
        )),
    );

    // hSetEncoding :: Handle -> TextEncoding -> IO ()
    env_chirho.bind_chirho(
        "hSetEncoding".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Handle".to_string()),
                TyChirho::ConChirho("TextEncoding".to_string()),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hPutStr :: Handle -> String -> IO ()
    env_chirho.bind_chirho(
        "hPutStr".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Handle".to_string()),
                TyChirho::string_chirho(),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hPutStrLn :: Handle -> String -> IO ()
    env_chirho.bind_chirho(
        "hPutStrLn".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::ConChirho("Handle".to_string()),
                TyChirho::string_chirho(),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hClose :: Handle -> IO ()
    env_chirho.bind_chirho(
        "hClose".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        )),
    );

    // hGetContents :: Handle -> IO String
    env_chirho.bind_chirho(
        "hGetContents".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::ConChirho("Handle".to_string()),
            TyChirho::io_chirho(TyChirho::string_chirho()),
        )),
    );

    // openFile :: FilePath -> IOMode -> IO Handle
    env_chirho.bind_chirho(
        "openFile".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::string_chirho(),
                TyChirho::ConChirho("IOMode".to_string()),
            ],
            TyChirho::io_chirho(TyChirho::ConChirho("Handle".to_string())),
        )),
    );

    // withFile / withBinaryFile :: FilePath -> IOMode -> (Handle -> IO a) -> IO a
    {
        let with_file_a_chirho = TyVarChirho(7810);
        let with_file_scheme_chirho = SchemeChirho {
            vars_chirho: vec![with_file_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::string_chirho(),
                    TyChirho::ConChirho("IOMode".to_string()),
                    TyChirho::fun_chirho(
                        TyChirho::ConChirho("Handle".to_string()),
                        TyChirho::io_chirho(TyChirho::VarChirho(with_file_a_chirho)),
                    ),
                ],
                TyChirho::io_chirho(TyChirho::VarChirho(with_file_a_chirho)),
            ),
        };
        env_chirho.bind_chirho("withFile".to_string(), with_file_scheme_chirho.clone());
        env_chirho.bind_chirho("withBinaryFile".to_string(), with_file_scheme_chirho);
    }

    // System.IO TextEncoding surface.
    //
    // Keep these seeded under module-qualified names only. Bare bindings like
    // `char8` or `latin1` collide with bytestring-builder exports and can pin
    // unrelated imports to the System.IO `TextEncoding` surface.
    for encoding_module_name_chirho in &[
        "System.IO",
        "GHC.IO.Encoding",
        "GHC.IO.Encoding.UTF8",
        "GHC.IO.Encoding.Latin1",
        "GHC.IO.Encoding.CodePage",
    ] {
        for text_encoding_name_chirho in &[
            "utf8",
            "utf8_bom",
            "utf16",
            "utf16le",
            "utf16be",
            "utf32",
            "utf32le",
            "utf32be",
            "latin1",
            "char8",
            "localeEncoding",
        ] {
            env_chirho.bind_chirho(
                format!("{encoding_module_name_chirho}.{text_encoding_name_chirho}"),
                SchemeChirho::mono_chirho(TyChirho::ConChirho("TextEncoding".to_string())),
            );
        }
        env_chirho.bind_chirho(
            format!("{encoding_module_name_chirho}.mkTextEncoding"),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                TyChirho::string_chirho(),
                TyChirho::io_chirho(TyChirho::ConChirho("TextEncoding".to_string())),
            )),
        );
        for (encoding_getter_name_chirho, encoding_result_ty_chirho) in [
            (
                "getLocaleEncoding",
                TyChirho::io_chirho(TyChirho::ConChirho("TextEncoding".to_string())),
            ),
            (
                "getFileSystemEncoding",
                TyChirho::io_chirho(TyChirho::ConChirho("TextEncoding".to_string())),
            ),
            (
                "getForeignEncoding",
                TyChirho::io_chirho(TyChirho::ConChirho("TextEncoding".to_string())),
            ),
        ] {
            env_chirho.bind_chirho(
                format!("{encoding_module_name_chirho}.{encoding_getter_name_chirho}"),
                SchemeChirho::mono_chirho(encoding_result_ty_chirho),
            );
        }
        for encoding_setter_name_chirho in &[
            "setLocaleEncoding",
            "setFileSystemEncoding",
            "setForeignEncoding",
        ] {
            env_chirho.bind_chirho(
                format!("{encoding_module_name_chirho}.{encoding_setter_name_chirho}"),
                SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                    TyChirho::ConChirho("TextEncoding".to_string()),
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                )),
            );
        }
    }

    // Data.Ord Down surface
    {
        let down_a_chirho = TyVarChirho(7600);
        let down_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Down".to_string())),
            Box::new(TyChirho::VarChirho(down_a_chirho)),
        );
        env_chirho.bind_chirho(
            "Down".to_string(),
            SchemeChirho {
                vars_chirho: vec![down_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(down_a_chirho),
                    down_ty_chirho.clone(),
                ),
            },
        );
        env_chirho.bind_chirho(
            "getDown".to_string(),
            SchemeChirho {
                vars_chirho: vec![down_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(down_ty_chirho, TyChirho::VarChirho(down_a_chirho)),
            },
        );
    }

    // Text.Printf record/enum surface used by QuickCheck
    {
        let fp_modifiers_chirho = TyVarChirho(7529);
        let fp_char_chirho = TyVarChirho(7530);
        let fp_rest_chirho = TyVarChirho(7531);
        env_chirho.bind_chirho(
            "FormatParse".to_string(),
            SchemeChirho {
                vars_chirho: vec![fp_modifiers_chirho, fp_char_chirho, fp_rest_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(fp_modifiers_chirho),
                        TyChirho::VarChirho(fp_char_chirho),
                        TyChirho::VarChirho(fp_rest_chirho),
                    ],
                    TyChirho::ConChirho("FormatParse".to_string()),
                ),
            },
        );
        env_chirho.bind_chirho(
            "fpModifiers".to_string(),
            SchemeChirho {
                vars_chirho: vec![fp_modifiers_chirho, fp_char_chirho, fp_rest_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ConChirho("FormatParse".to_string()),
                    TyChirho::VarChirho(fp_modifiers_chirho),
                ),
            },
        );
        env_chirho.bind_chirho(
            "fpChar".to_string(),
            SchemeChirho {
                vars_chirho: vec![fp_modifiers_chirho, fp_char_chirho, fp_rest_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ConChirho("FormatParse".to_string()),
                    TyChirho::VarChirho(fp_char_chirho),
                ),
            },
        );
        env_chirho.bind_chirho(
            "fpRest".to_string(),
            SchemeChirho {
                vars_chirho: vec![fp_modifiers_chirho, fp_char_chirho, fp_rest_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ConChirho("FormatParse".to_string()),
                    TyChirho::VarChirho(fp_rest_chirho),
                ),
            },
        );
    }
    {
        let fmt_width_chirho = TyVarChirho(7532);
        let fmt_precision_chirho = TyVarChirho(7533);
        let fmt_adjust_chirho = TyVarChirho(7534);
        let fmt_sign_chirho = TyVarChirho(7535);
        let fmt_alternate_chirho = TyVarChirho(7536);
        let fmt_modifiers_chirho = TyVarChirho(7537);
        let fmt_char_chirho = TyVarChirho(7538);
        env_chirho.bind_chirho(
            "FieldFormat".to_string(),
            SchemeChirho {
                vars_chirho: vec![
                    fmt_width_chirho,
                    fmt_precision_chirho,
                    fmt_adjust_chirho,
                    fmt_sign_chirho,
                    fmt_alternate_chirho,
                    fmt_modifiers_chirho,
                    fmt_char_chirho,
                ],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(fmt_width_chirho),
                        TyChirho::VarChirho(fmt_precision_chirho),
                        TyChirho::VarChirho(fmt_adjust_chirho),
                        TyChirho::VarChirho(fmt_sign_chirho),
                        TyChirho::VarChirho(fmt_alternate_chirho),
                        TyChirho::VarChirho(fmt_modifiers_chirho),
                        TyChirho::VarChirho(fmt_char_chirho),
                    ],
                    TyChirho::ConChirho("FieldFormat".to_string()),
                ),
            },
        );
        for (selector_name_chirho, result_var_chirho) in [
            ("fmtWidth", fmt_width_chirho),
            ("fmtPrecision", fmt_precision_chirho),
            ("fmtAdjust", fmt_adjust_chirho),
            ("fmtSign", fmt_sign_chirho),
            ("fmtAlternate", fmt_alternate_chirho),
            ("fmtModifiers", fmt_modifiers_chirho),
            ("fmtChar", fmt_char_chirho),
        ] {
            env_chirho.bind_chirho(
                selector_name_chirho.to_string(),
                SchemeChirho {
                    vars_chirho: vec![
                        fmt_width_chirho,
                        fmt_precision_chirho,
                        fmt_adjust_chirho,
                        fmt_sign_chirho,
                        fmt_alternate_chirho,
                        fmt_modifiers_chirho,
                        fmt_char_chirho,
                    ],
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_chirho(
                        TyChirho::ConChirho("FieldFormat".to_string()),
                        TyChirho::VarChirho(result_var_chirho),
                    ),
                },
            );
        }
    }
    for (constructor_name_chirho, type_name_chirho) in [
        ("LeftAdjust", "FormatAdjustment"),
        ("ZeroPad", "FormatAdjustment"),
        ("SignPlus", "FormatSign"),
        ("SignSpace", "FormatSign"),
    ] {
        env_chirho.bind_chirho(
            constructor_name_chirho.to_string(),
            SchemeChirho::mono_chirho(TyChirho::ConChirho(type_name_chirho.to_string())),
        );
    }

    // Data.IORef functions with type schemes
    // readIORef :: forall a. IORef a -> IO a
    {
        let a_chirho = TyVarChirho(7520);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "readIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    ioref_a_chirho,
                    TyChirho::io_chirho(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // writeIORef :: forall a. IORef a -> a -> IO ()
    {
        let a_chirho = TyVarChirho(7521);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "writeIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![ioref_a_chirho, TyChirho::VarChirho(a_chirho)],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // modifyIORef :: forall a. IORef a -> (a -> a) -> IO ()
    {
        let a_chirho = TyVarChirho(7522);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "modifyIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        ioref_a_chirho,
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(a_chirho),
                        ),
                    ],
                    TyChirho::io_chirho(TyChirho::unit_chirho()),
                ),
            },
        );
    }

    // newIORef :: forall a. a -> IO (IORef a)
    {
        let a_chirho = TyVarChirho(7523);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "newIORef".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::io_chirho(ioref_a_chirho),
                ),
            },
        );
    }

    // atomicModifyIORef2Lazy :: forall a b. IORef a -> (a -> (a, b)) -> IO (a, (a, b))
    {
        let a_chirho = TyVarChirho(7524);
        let b_chirho = TyVarChirho(7525);
        let ioref_a_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("IORef".to_string())),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        let pair_ab_chirho = TyChirho::TupleChirho(vec![
            TyChirho::VarChirho(a_chirho),
            TyChirho::VarChirho(b_chirho),
        ]);
        env_chirho.bind_chirho(
            "atomicModifyIORef2Lazy".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        ioref_a_chirho,
                        TyChirho::fun_chirho(TyChirho::VarChirho(a_chirho), pair_ab_chirho.clone()),
                    ],
                    TyChirho::io_chirho(TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(a_chirho),
                        pair_ab_chirho,
                    ])),
                ),
            },
        );
    }

    // mapAccumL :: forall a b c. (a -> b -> (a, c)) -> a -> [b] -> (a, [c])
    {
        let a_chirho = TyVarChirho(7530);
        let b_chirho = TyVarChirho(7531);
        let c_chirho = TyVarChirho(7532);
        env_chirho.bind_chirho(
            "mapAccumL".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::TupleChirho(vec![
                                TyChirho::VarChirho(a_chirho),
                                TyChirho::VarChirho(c_chirho),
                            ]),
                        ),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho))),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(c_chirho))),
                    ]),
                ),
            },
        );
    }

    // mapAccumR :: forall a b c. (a -> b -> (a, c)) -> a -> [b] -> (a, [c])
    {
        let a_chirho = TyVarChirho(7533);
        let b_chirho = TyVarChirho(7534);
        let c_chirho = TyVarChirho(7535);
        env_chirho.bind_chirho(
            "mapAccumR".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(a_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::TupleChirho(vec![
                                TyChirho::VarChirho(a_chirho),
                                TyChirho::VarChirho(c_chirho),
                            ]),
                        ),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(b_chirho))),
                    ],
                    TyChirho::TupleChirho(vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(c_chirho))),
                    ]),
                ),
            },
        );
    }

    // StaticPointers: `static` is treated as `forall a. a -> StaticPtr a`
    // so that `static expr` type-checks and returns `StaticPtr a`.
    let static_v_chirho = TyVarChirho(1800);
    env_chirho.bind_chirho(
        "static".to_string(),
        SchemeChirho {
            vars_chirho: vec![static_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(static_v_chirho),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("StaticPtr".to_string())),
                    Box::new(TyChirho::VarChirho(static_v_chirho)),
                ),
            ),
        },
    );

    // Test.Hspec DSL stubs used by package test suites.
    let hspec_spec_ty_chirho = TyChirho::io_chirho(TyChirho::unit_chirho());
    let hspec_a_chirho = TyVarChirho(1801);
    env_chirho.bind_chirho(
        "hspec".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            hspec_spec_ty_chirho.clone(),
            hspec_spec_ty_chirho.clone(),
        )),
    );
    env_chirho.bind_chirho(
        "describe".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), hspec_spec_ty_chirho.clone()],
            hspec_spec_ty_chirho.clone(),
        )),
    );
    env_chirho.bind_chirho(
        "it".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::string_chirho(), hspec_spec_ty_chirho.clone()],
            hspec_spec_ty_chirho.clone(),
        )),
    );
    env_chirho.bind_chirho(
        "shouldBe".to_string(),
        SchemeChirho {
            vars_chirho: vec![hspec_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(hspec_a_chirho),
                    TyChirho::VarChirho(hspec_a_chirho),
                ],
                hspec_spec_ty_chirho,
            ),
        },
    );

    // List cons constructor: (:) :: forall a. a -> [a] -> [a]
    let cons_v_chirho = TyVarChirho(1802);
    env_chirho.bind_chirho(
        ":".to_string(),
        SchemeChirho {
            vars_chirho: vec![cons_v_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(cons_v_chirho),
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cons_v_chirho))),
                ],
                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(cons_v_chirho))),
            ),
        },
    );
}
