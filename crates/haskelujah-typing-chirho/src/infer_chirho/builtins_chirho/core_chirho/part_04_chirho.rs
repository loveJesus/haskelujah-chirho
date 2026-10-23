// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 04 of 14: original lines 10634-11592 of
//! `seed_builtins_chirho`, in order (58 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_04_chirho(env_chirho: &mut TyEnvChirho) {
    // System.Random / System.Random.SplitMix
    let randomgen_g_chirho = TyVarChirho(1690);
    let split_pair_ty_chirho = TyChirho::TupleChirho(vec![
        TyChirho::VarChirho(randomgen_g_chirho),
        TyChirho::VarChirho(randomgen_g_chirho),
    ]);
    env_chirho.bind_chirho(
        "split".to_string(),
        SchemeChirho {
            vars_chirho: vec![randomgen_g_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "RandomGen".to_string(),
                ty_chirho: TyChirho::VarChirho(randomgen_g_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(randomgen_g_chirho),
                split_pair_ty_chirho.clone(),
            ),
        },
    );
    env_chirho.bind_chirho(
        "splitGen".to_string(),
        SchemeChirho {
            vars_chirho: vec![randomgen_g_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "SplitGen".to_string(),
                ty_chirho: TyChirho::VarChirho(randomgen_g_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(randomgen_g_chirho),
                split_pair_ty_chirho,
            ),
        },
    );
    env_chirho.bind_chirho(
        "next".to_string(),
        SchemeChirho {
            vars_chirho: vec![randomgen_g_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "RandomGen".to_string(),
                ty_chirho: TyChirho::VarChirho(randomgen_g_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(randomgen_g_chirho),
                TyChirho::TupleChirho(vec![
                    TyChirho::int_chirho(),
                    TyChirho::VarChirho(randomgen_g_chirho),
                ]),
            ),
        },
    );
    env_chirho.bind_chirho(
        "genRange".to_string(),
        SchemeChirho {
            vars_chirho: vec![randomgen_g_chirho],
            preds_chirho: vec![SchemePredChirho {
                class_name_chirho: "RandomGen".to_string(),
                ty_chirho: TyChirho::VarChirho(randomgen_g_chirho),
                extra_tys_chirho: vec![],
            }],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(randomgen_g_chirho),
                TyChirho::TupleChirho(vec![TyChirho::int_chirho(), TyChirho::int_chirho()]),
            ),
        },
    );
    env_chirho.bind_chirho(
        "mkStdGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::ConChirho("StdGen".to_string()),
        )),
    );
    env_chirho.bind_chirho(
        "newStdGen".to_string(),
        SchemeChirho::mono_chirho(TyChirho::io_chirho(TyChirho::ConChirho(
            "StdGen".to_string(),
        ))),
    );
    let random_a_chirho = TyVarChirho(1691);
    let random_range_ty_chirho = TyChirho::TupleChirho(vec![
        TyChirho::VarChirho(random_a_chirho),
        TyChirho::VarChirho(random_a_chirho),
    ]);
    let random_result_ty_chirho = TyChirho::TupleChirho(vec![
        TyChirho::VarChirho(random_a_chirho),
        TyChirho::VarChirho(randomgen_g_chirho),
    ]);
    let random_preds_chirho = vec![
        SchemePredChirho {
            class_name_chirho: "Random".to_string(),
            ty_chirho: TyChirho::VarChirho(random_a_chirho),
            extra_tys_chirho: vec![],
        },
        SchemePredChirho {
            class_name_chirho: "RandomGen".to_string(),
            ty_chirho: TyChirho::VarChirho(randomgen_g_chirho),
            extra_tys_chirho: vec![],
        },
    ];
    env_chirho.bind_chirho(
        "random".to_string(),
        SchemeChirho {
            vars_chirho: vec![random_a_chirho, randomgen_g_chirho],
            preds_chirho: random_preds_chirho.clone(),
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(randomgen_g_chirho),
                random_result_ty_chirho.clone(),
            ),
        },
    );
    env_chirho.bind_chirho(
        "randomR".to_string(),
        SchemeChirho {
            vars_chirho: vec![random_a_chirho, randomgen_g_chirho],
            preds_chirho: random_preds_chirho,
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    random_range_ty_chirho.clone(),
                    TyChirho::VarChirho(randomgen_g_chirho),
                ],
                random_result_ty_chirho,
            ),
        },
    );
    let random_io_pred_chirho = SchemePredChirho {
        class_name_chirho: "Random".to_string(),
        ty_chirho: TyChirho::VarChirho(random_a_chirho),
        extra_tys_chirho: vec![],
    };
    env_chirho.bind_chirho(
        "randomIO".to_string(),
        SchemeChirho {
            vars_chirho: vec![random_a_chirho],
            preds_chirho: vec![random_io_pred_chirho.clone()],
            ty_chirho: TyChirho::io_chirho(TyChirho::VarChirho(random_a_chirho)),
        },
    );
    env_chirho.bind_chirho(
        "randomRIO".to_string(),
        SchemeChirho {
            vars_chirho: vec![random_a_chirho],
            preds_chirho: vec![random_io_pred_chirho],
            ty_chirho: TyChirho::fun_chirho(
                random_range_ty_chirho,
                TyChirho::io_chirho(TyChirho::VarChirho(random_a_chirho)),
            ),
        },
    );

    let foreign_ptr_a_chirho = TyVarChirho(1692);
    let foreign_ptr_b_chirho = TyVarChirho(1693);
    let foreign_ptr_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("ForeignPtr".to_string())),
        Box::new(TyChirho::VarChirho(foreign_ptr_a_chirho)),
    );
    let ptr_ty_chirho = TyChirho::AppChirho(
        Box::new(TyChirho::ConChirho("Ptr".to_string())),
        Box::new(TyChirho::VarChirho(foreign_ptr_a_chirho)),
    );
    let io_foreign_ptr_result_ty_chirho =
        TyChirho::io_chirho(TyChirho::VarChirho(foreign_ptr_b_chirho));
    let with_foreign_ptr_scheme_chirho = SchemeChirho {
        vars_chirho: vec![foreign_ptr_a_chirho, foreign_ptr_b_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![
                foreign_ptr_ty_chirho.clone(),
                TyChirho::fun_chirho(ptr_ty_chirho, io_foreign_ptr_result_ty_chirho.clone()),
            ],
            io_foreign_ptr_result_ty_chirho,
        ),
    };
    env_chirho.bind_chirho(
        "withForeignPtr".to_string(),
        with_foreign_ptr_scheme_chirho.clone(),
    );
    env_chirho.bind_chirho(
        "unsafeWithForeignPtr".to_string(),
        with_foreign_ptr_scheme_chirho,
    );

    let malloc_plain_foreign_ptr_bytes_scheme_chirho = SchemeChirho {
        vars_chirho: vec![foreign_ptr_a_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_chirho(
            TyChirho::int_chirho(),
            TyChirho::io_chirho(foreign_ptr_ty_chirho.clone()),
        ),
    };
    for name_chirho in [
        "mallocPlainForeignPtrBytes",
        "Data.ByteString.Internal.mallocPlainForeignPtrBytes",
        "GHC.ForeignPtr.mallocPlainForeignPtrBytes",
        "Foreign.ForeignPtr.mallocPlainForeignPtrBytes",
        "GHC.ForeignPtr.Internal.mallocPlainForeignPtrBytes",
    ] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            malloc_plain_foreign_ptr_bytes_scheme_chirho.clone(),
        );
    }

    let plus_foreign_ptr_scheme_chirho = SchemeChirho {
        vars_chirho: vec![foreign_ptr_a_chirho],
        preds_chirho: vec![],
        ty_chirho: TyChirho::fun_n_chirho(
            vec![foreign_ptr_ty_chirho.clone(), TyChirho::int_chirho()],
            foreign_ptr_ty_chirho.clone(),
        ),
    };
    for name_chirho in [
        "plusForeignPtr",
        "Data.ByteString.Internal.plusForeignPtr",
        "GHC.ForeignPtr.plusForeignPtr",
        "Foreign.ForeignPtr.plusForeignPtr",
        "GHC.ForeignPtr.Internal.plusForeignPtr",
    ] {
        env_chirho.bind_chirho(
            name_chirho.to_string(),
            plus_foreign_ptr_scheme_chirho.clone(),
        );
    }

    {
        let addr_prim_ty_chirho = TyChirho::ConChirho("Addr#".to_string());
        let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
        let string_ty_chirho = TyChirho::ListChirho(Box::new(char_ty_chirho.clone()));
        let unpack_foldr_a_chirho = TyVarChirho(1698);
        let unpack_cstring_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            addr_prim_ty_chirho.clone(),
            string_ty_chirho.clone(),
        ));
        let unpack_append_cstring_scheme_chirho =
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![addr_prim_ty_chirho.clone(), string_ty_chirho.clone()],
                string_ty_chirho.clone(),
            ));
        let unpack_foldr_cstring_scheme_chirho = SchemeChirho {
            vars_chirho: vec![unpack_foldr_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    addr_prim_ty_chirho.clone(),
                    TyChirho::fun_n_chirho(
                        vec![
                            char_ty_chirho.clone(),
                            TyChirho::VarChirho(unpack_foldr_a_chirho),
                        ],
                        TyChirho::VarChirho(unpack_foldr_a_chirho),
                    ),
                    TyChirho::VarChirho(unpack_foldr_a_chirho),
                ],
                TyChirho::VarChirho(unpack_foldr_a_chirho),
            ),
        };
        for name_chirho in [
            "unpackCString#",
            "GHC.CString.unpackCString#",
            "GHC.Base.unpackCString#",
            "GHC.Exts.unpackCString#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unpack_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in [
            "unpackAppendCString#",
            "GHC.CString.unpackAppendCString#",
            "GHC.Base.unpackAppendCString#",
            "GHC.Exts.unpackAppendCString#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unpack_append_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in [
            "unpackFoldrCString#",
            "GHC.CString.unpackFoldrCString#",
            "GHC.Base.unpackFoldrCString#",
            "GHC.Exts.unpackFoldrCString#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unpack_foldr_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in [
            "unpackCStringUtf8#",
            "GHC.CString.unpackCStringUtf8#",
            "GHC.Base.unpackCStringUtf8#",
            "GHC.Exts.unpackCStringUtf8#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unpack_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in [
            "unpackAppendCStringUtf8#",
            "GHC.CString.unpackAppendCStringUtf8#",
            "GHC.Base.unpackAppendCStringUtf8#",
            "GHC.Exts.unpackAppendCStringUtf8#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unpack_append_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in [
            "unpackFoldrCStringUtf8#",
            "GHC.CString.unpackFoldrCStringUtf8#",
            "GHC.Base.unpackFoldrCStringUtf8#",
            "GHC.Exts.unpackFoldrCStringUtf8#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unpack_foldr_cstring_scheme_chirho.clone(),
            );
        }

        let foreign_cstring_a_chirho = TyVarChirho(1699);
        let c_char_ty_chirho = TyChirho::ConChirho("CChar".to_string());
        let c_string_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(c_char_ty_chirho),
        );
        let c_string_len_ty_chirho =
            TyChirho::TupleChirho(vec![c_string_ty_chirho.clone(), TyChirho::int_chirho()]);
        let text_encoding_ty_chirho = TyChirho::ConChirho("TextEncoding".to_string());
        let foreign_string_result_ty_chirho =
            TyChirho::io_chirho(TyChirho::VarChirho(foreign_cstring_a_chirho));

        let with_cstring_scheme_chirho = SchemeChirho {
            vars_chirho: vec![foreign_cstring_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(
                        c_string_ty_chirho.clone(),
                        foreign_string_result_ty_chirho.clone(),
                    ),
                ],
                foreign_string_result_ty_chirho.clone(),
            ),
        };
        let with_cstring_len_scheme_chirho = SchemeChirho {
            vars_chirho: vec![foreign_cstring_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(
                        c_string_len_ty_chirho.clone(),
                        foreign_string_result_ty_chirho.clone(),
                    ),
                ],
                foreign_string_result_ty_chirho.clone(),
            ),
        };
        let with_encoded_cstring_scheme_chirho = SchemeChirho {
            vars_chirho: vec![foreign_cstring_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    text_encoding_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(
                        c_string_ty_chirho.clone(),
                        foreign_string_result_ty_chirho.clone(),
                    ),
                ],
                foreign_string_result_ty_chirho.clone(),
            ),
        };
        let with_encoded_cstring_len_scheme_chirho = SchemeChirho {
            vars_chirho: vec![foreign_cstring_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    text_encoding_ty_chirho.clone(),
                    TyChirho::string_chirho(),
                    TyChirho::fun_chirho(
                        c_string_len_ty_chirho.clone(),
                        foreign_string_result_ty_chirho.clone(),
                    ),
                ],
                foreign_string_result_ty_chirho.clone(),
            ),
        };
        let peek_cstring_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            c_string_ty_chirho.clone(),
            TyChirho::io_chirho(TyChirho::string_chirho()),
        ));
        let peek_cstring_len_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            c_string_len_ty_chirho.clone(),
            TyChirho::io_chirho(TyChirho::string_chirho()),
        ));
        let peek_encoded_cstring_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![text_encoding_ty_chirho.clone(), c_string_ty_chirho.clone()],
            TyChirho::io_chirho(TyChirho::string_chirho()),
        ));
        let peek_encoded_cstring_len_scheme_chirho =
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![
                    text_encoding_ty_chirho.clone(),
                    c_string_len_ty_chirho.clone(),
                ],
                TyChirho::io_chirho(TyChirho::string_chirho()),
            ));
        let new_cstring_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(c_string_ty_chirho.clone()),
        ));
        let new_cstring_len_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            TyChirho::string_chirho(),
            TyChirho::io_chirho(c_string_len_ty_chirho.clone()),
        ));
        let new_encoded_cstring_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![text_encoding_ty_chirho.clone(), TyChirho::string_chirho()],
            TyChirho::io_chirho(c_string_ty_chirho.clone()),
        ));
        let new_encoded_cstring_len_scheme_chirho =
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![text_encoding_ty_chirho, TyChirho::string_chirho()],
                TyChirho::io_chirho(c_string_len_ty_chirho.clone()),
            ));

        for name_chirho in ["withCString", "Foreign.C.String.withCString"] {
            env_chirho.bind_chirho(name_chirho.to_string(), with_cstring_scheme_chirho.clone());
        }
        for name_chirho in ["withCStringLen", "Foreign.C.String.withCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                with_cstring_len_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["peekCString", "Foreign.C.String.peekCString"] {
            env_chirho.bind_chirho(name_chirho.to_string(), peek_cstring_scheme_chirho.clone());
        }
        for name_chirho in ["peekCStringLen", "Foreign.C.String.peekCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                peek_cstring_len_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["newCString", "Foreign.C.String.newCString"] {
            env_chirho.bind_chirho(name_chirho.to_string(), new_cstring_scheme_chirho.clone());
        }
        for name_chirho in ["newCStringLen", "Foreign.C.String.newCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                new_cstring_len_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["GHC.Foreign.withCString"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                with_encoded_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["GHC.Foreign.withCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                with_encoded_cstring_len_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["GHC.Foreign.peekCString"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                peek_encoded_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["GHC.Foreign.peekCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                peek_encoded_cstring_len_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["GHC.Foreign.newCString"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                new_encoded_cstring_scheme_chirho.clone(),
            );
        }
        for name_chirho in ["GHC.Foreign.newCStringLen"] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                new_encoded_cstring_len_scheme_chirho.clone(),
            );
        }
    }

    {
        let byteorder_ty_chirho = TyChirho::ConChirho("ByteOrder".to_string());
        let byteorder_scheme_chirho = SchemeChirho::mono_chirho(byteorder_ty_chirho);
        for name_chirho in ["LittleEndian", "BigEndian"] {
            env_chirho.bind_chirho(name_chirho.to_string(), byteorder_scheme_chirho.clone());
        }
        for name_chirho in [
            "GHC.ByteOrder.LittleEndian",
            "GHC.ByteOrder.BigEndian",
            "targetByteOrder",
            "GHC.ByteOrder.targetByteOrder",
        ] {
            env_chirho.bind_chirho(name_chirho.to_string(), byteorder_scheme_chirho.clone());
        }
    }

    {
        let word8_ty_chirho = TyChirho::ConChirho("Word8".to_string());
        let char_ty_chirho = TyChirho::ConChirho("Char".to_string());
        let c2w_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            char_ty_chirho.clone(),
            word8_ty_chirho.clone(),
        ));
        env_chirho.bind_chirho("c2w".to_string(), c2w_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.ByteString.Internal.c2w".to_string(),
            c2w_scheme_chirho,
        );

        let w2c_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_chirho(
            word8_ty_chirho.clone(),
            char_ty_chirho,
        ));
        env_chirho.bind_chirho("w2c".to_string(), w2c_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.ByteString.Internal.w2c".to_string(),
            w2c_scheme_chirho,
        );

        let memset_a_chirho = TyVarChirho(1697);
        let memset_ptr_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(TyChirho::VarChirho(memset_a_chirho)),
        );
        let memset_scheme_chirho = SchemeChirho {
            vars_chirho: vec![memset_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    memset_ptr_ty_chirho.clone(),
                    TyChirho::int_chirho(),
                    TyChirho::int_chirho(),
                ],
                TyChirho::io_chirho(memset_ptr_ty_chirho),
            ),
        };
        for name_chirho in [
            "memset",
            "Data.ByteString.Internal.memset",
            "GHC.ForeignPtr.memset",
            "Foreign.Marshal.memset",
            "Foreign.Marshal.Utils.memset",
        ] {
            env_chirho.bind_chirho(name_chirho.to_string(), memset_scheme_chirho.clone());
        }

        let memcpy_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Ptr".to_string())),
                    Box::new(word8_ty_chirho.clone()),
                ),
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Ptr".to_string())),
                    Box::new(word8_ty_chirho),
                ),
                TyChirho::int_chirho(),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        ));
        env_chirho.bind_chirho("memcpy".to_string(), memcpy_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.ByteString.Internal.memcpy".to_string(),
            memcpy_scheme_chirho,
        );
    }

    {
        let storable_a_chirho = TyVarChirho(1694);
        let storable_ptr_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(TyChirho::VarChirho(storable_a_chirho)),
        );
        let storable_pred_chirho = SchemePredChirho {
            class_name_chirho: "Storable".to_string(),
            ty_chirho: TyChirho::VarChirho(storable_a_chirho),
            extra_tys_chirho: vec![],
        };

        let size_of_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho],
            preds_chirho: vec![storable_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(storable_a_chirho),
                TyChirho::int_chirho(),
            ),
        };
        env_chirho.bind_chirho("sizeOf".to_string(), size_of_scheme_chirho.clone());
        env_chirho.bind_chirho("Foreign.Storable.sizeOf".to_string(), size_of_scheme_chirho);

        let alignment_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho],
            preds_chirho: vec![storable_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(storable_a_chirho),
                TyChirho::int_chirho(),
            ),
        };
        env_chirho.bind_chirho("alignment".to_string(), alignment_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Foreign.Storable.alignment".to_string(),
            alignment_scheme_chirho,
        );

        let peek_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho],
            preds_chirho: vec![storable_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_chirho(
                storable_ptr_ty_chirho.clone(),
                TyChirho::io_chirho(TyChirho::VarChirho(storable_a_chirho)),
            ),
        };
        env_chirho.bind_chirho("peek".to_string(), peek_scheme_chirho.clone());
        env_chirho.bind_chirho("Foreign.Storable.peek".to_string(), peek_scheme_chirho);

        let storable_b_chirho = TyVarChirho(1695);
        let raw_ptr_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(TyChirho::VarChirho(storable_b_chirho)),
        );

        let peek_byte_off_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho, storable_b_chirho],
            preds_chirho: vec![storable_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![raw_ptr_ty_chirho.clone(), TyChirho::int_chirho()],
                TyChirho::io_chirho(TyChirho::VarChirho(storable_a_chirho)),
            ),
        };
        env_chirho.bind_chirho(
            "peekByteOff".to_string(),
            peek_byte_off_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Foreign.Storable.peekByteOff".to_string(),
            peek_byte_off_scheme_chirho,
        );

        let peek_elem_off_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho],
            preds_chirho: vec![storable_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![storable_ptr_ty_chirho.clone(), TyChirho::int_chirho()],
                TyChirho::io_chirho(TyChirho::VarChirho(storable_a_chirho)),
            ),
        };
        env_chirho.bind_chirho(
            "peekElemOff".to_string(),
            peek_elem_off_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Foreign.Storable.peekElemOff".to_string(),
            peek_elem_off_scheme_chirho,
        );

        let poke_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho],
            preds_chirho: vec![storable_pred_chirho],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    storable_ptr_ty_chirho.clone(),
                    TyChirho::VarChirho(storable_a_chirho),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        };
        env_chirho.bind_chirho("poke".to_string(), poke_scheme_chirho.clone());
        env_chirho.bind_chirho("Foreign.Storable.poke".to_string(), poke_scheme_chirho);

        let storable_pred_chirho = SchemePredChirho {
            class_name_chirho: "Storable".to_string(),
            ty_chirho: TyChirho::VarChirho(storable_a_chirho),
            extra_tys_chirho: vec![],
        };
        let poke_byte_off_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho, storable_b_chirho],
            preds_chirho: vec![storable_pred_chirho.clone()],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    raw_ptr_ty_chirho,
                    TyChirho::int_chirho(),
                    TyChirho::VarChirho(storable_a_chirho),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        };
        env_chirho.bind_chirho(
            "pokeByteOff".to_string(),
            poke_byte_off_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Foreign.Storable.pokeByteOff".to_string(),
            poke_byte_off_scheme_chirho,
        );

        let poke_elem_off_scheme_chirho = SchemeChirho {
            vars_chirho: vec![storable_a_chirho],
            preds_chirho: vec![storable_pred_chirho],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    storable_ptr_ty_chirho,
                    TyChirho::int_chirho(),
                    TyChirho::VarChirho(storable_a_chirho),
                ],
                TyChirho::io_chirho(TyChirho::unit_chirho()),
            ),
        };
        env_chirho.bind_chirho(
            "pokeElemOff".to_string(),
            poke_elem_off_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Foreign.Storable.pokeElemOff".to_string(),
            poke_elem_off_scheme_chirho,
        );
    }

    {
        let buffer_pool_ty_chirho = TyChirho::ConChirho("BufferPool".to_string());
        let ptr_word8_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(TyChirho::ConChirho("Word8".to_string())),
        );
        let socket_ty_chirho = TyChirho::ConChirho("Socket".to_string());

        let new_buffer_pool_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![TyChirho::int_chirho(), TyChirho::int_chirho()],
            TyChirho::io_chirho(buffer_pool_ty_chirho.clone()),
        ));
        env_chirho.bind_chirho(
            "newBufferPool".to_string(),
            new_buffer_pool_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Network.Socket.BufferPool.newBufferPool".to_string(),
            new_buffer_pool_scheme_chirho,
        );

        let copy_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                ptr_word8_ty_chirho.clone(),
                TyChirho::ConChirho("ByteString".to_string()),
            ],
            TyChirho::io_chirho(ptr_word8_ty_chirho.clone()),
        ));
        env_chirho.bind_chirho("copy".to_string(), copy_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Network.Socket.BufferPool.copy".to_string(),
            copy_scheme_chirho,
        );

        let receive_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![socket_ty_chirho, buffer_pool_ty_chirho],
            TyChirho::io_chirho(TyChirho::ConChirho("ByteString".to_string())),
        ));
        env_chirho.bind_chirho("receive".to_string(), receive_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Network.Socket.BufferPool.receive".to_string(),
            receive_scheme_chirho,
        );

        let write_word8_off_ptr_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![
                ptr_word8_ty_chirho,
                TyChirho::int_chirho(),
                TyChirho::ConChirho("Word8".to_string()),
            ],
            TyChirho::io_chirho(TyChirho::unit_chirho()),
        ));
        env_chirho.bind_chirho(
            "writeWord8OffPtr".to_string(),
            write_word8_off_ptr_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "GHC.Storable.writeWord8OffPtr".to_string(),
            write_word8_off_ptr_scheme_chirho,
        );
    }

    {
        let cast_fun_ptr_from_a_chirho = TyVarChirho(1695);
        let cast_fun_ptr_to_a_chirho = TyVarChirho(1696);
        let cast_fun_ptr_input_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("FunPtr".to_string())),
            Box::new(TyChirho::VarChirho(cast_fun_ptr_from_a_chirho)),
        );
        let cast_fun_ptr_result_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("Ptr".to_string())),
            Box::new(TyChirho::VarChirho(cast_fun_ptr_to_a_chirho)),
        );
        let cast_fun_ptr_scheme_chirho = SchemeChirho {
            vars_chirho: vec![cast_fun_ptr_from_a_chirho, cast_fun_ptr_to_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                cast_fun_ptr_input_ty_chirho,
                cast_fun_ptr_result_ty_chirho,
            ),
        };
        env_chirho.bind_chirho(
            "castFunPtrToPtr".to_string(),
            cast_fun_ptr_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Foreign.Ptr.castFunPtrToPtr".to_string(),
            cast_fun_ptr_scheme_chirho,
        );
    }

    {
        let text_array_a_chirho = TyVarChirho(1697);
        let text_array_a_scheme_chirho = SchemeChirho {
            vars_chirho: vec![text_array_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(text_array_a_chirho),
                TyChirho::ConChirho("ByteArray#".to_string()),
            ),
        };
        env_chirho.bind_chirho("aBA".to_string(), text_array_a_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.Text.Array.aBA".to_string(),
            text_array_a_scheme_chirho,
        );
    }

    {
        let text_array_ty_chirho = TyChirho::ConChirho("Array".to_string());
        let text_word8_ty_chirho = TyChirho::ConChirho("Word8".to_string());
        let text_array_state_chirho = TyVarChirho(1698);
        let text_marray_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MArray".to_string())),
            Box::new(TyChirho::VarChirho(text_array_state_chirho)),
        );
        let text_st_marray_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(text_array_state_chirho)),
            )),
            Box::new(text_marray_ty_chirho.clone()),
        );
        let text_st_array_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(text_array_state_chirho)),
            )),
            Box::new(text_array_ty_chirho.clone()),
        );
        let text_st_unit_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho("ST".to_string())),
                Box::new(TyChirho::VarChirho(text_array_state_chirho)),
            )),
            Box::new(TyChirho::unit_chirho()),
        );

        env_chirho.bind_chirho(
            "empty".to_string(),
            SchemeChirho::mono_chirho(text_array_ty_chirho.clone()),
        );
        env_chirho.bind_chirho(
            "Data.Text.Array.empty".to_string(),
            SchemeChirho::mono_chirho(text_array_ty_chirho.clone()),
        );

        let text_array_new_scheme_chirho = SchemeChirho {
            vars_chirho: vec![text_array_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::int_chirho(),
                text_st_marray_ty_chirho.clone(),
            ),
        };
        env_chirho.bind_chirho("new".to_string(), text_array_new_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.Text.Array.new".to_string(),
            text_array_new_scheme_chirho,
        );

        let text_array_run_scheme_chirho = SchemeChirho {
            vars_chirho: vec![text_array_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                text_st_marray_ty_chirho.clone(),
                text_array_ty_chirho.clone(),
            ),
        };
        env_chirho.bind_chirho("run".to_string(), text_array_run_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.Text.Array.run".to_string(),
            text_array_run_scheme_chirho,
        );

        let text_array_copyi_scheme_chirho = SchemeChirho {
            vars_chirho: vec![text_array_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::int_chirho(),
                    text_marray_ty_chirho.clone(),
                    TyChirho::int_chirho(),
                    text_array_ty_chirho.clone(),
                    TyChirho::int_chirho(),
                ],
                text_st_unit_ty_chirho,
            ),
        };
        env_chirho.bind_chirho("copyI".to_string(), text_array_copyi_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Data.Text.Array.copyI".to_string(),
            text_array_copyi_scheme_chirho,
        );

        let text_array_unsafe_freeze_scheme_chirho = SchemeChirho {
            vars_chirho: vec![text_array_state_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(text_marray_ty_chirho, text_st_array_ty_chirho),
        };
        env_chirho.bind_chirho(
            "unsafeFreeze".to_string(),
            text_array_unsafe_freeze_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Data.Text.Array.unsafeFreeze".to_string(),
            text_array_unsafe_freeze_scheme_chirho,
        );

        let text_array_unsafe_index_scheme_chirho =
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![text_array_ty_chirho, TyChirho::int_chirho()],
                text_word8_ty_chirho,
            ));
        env_chirho.bind_chirho(
            "unsafeIndex".to_string(),
            text_array_unsafe_index_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Data.Text.Array.unsafeIndex".to_string(),
            text_array_unsafe_index_scheme_chirho,
        );
    }
}
