// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin type seeds, part 12 of 14: original lines 18235-19192 of
//! `seed_builtins_chirho`, in order (50 named binds).

use crate::env_chirho::TyEnvChirho;
use crate::infer_chirho::mk_type_rep_ty_chirho;
use crate::ty_chirho::{SchemeChirho, SchemePredChirho, TyChirho, TyVarChirho};

pub(super) fn seed_part_12_chirho(env_chirho: &mut TyEnvChirho) {
    // tails :: forall a. [a] -> [[a]]
    {
        let tl_a_chirho = TyVarChirho(4130);
        env_chirho.bind_chirho(
            "tails".to_string(),
            SchemeChirho {
                vars_chirho: vec![tl_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(tl_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                        TyChirho::VarChirho(tl_a_chirho),
                    )))),
                ),
            },
        );
    }

    // inits :: forall a. [a] -> [[a]]
    {
        let in_a_chirho = TyVarChirho(4131);
        env_chirho.bind_chirho(
            "inits".to_string(),
            SchemeChirho {
                vars_chirho: vec![in_a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::VarChirho(in_a_chirho))),
                    TyChirho::ListChirho(Box::new(TyChirho::ListChirho(Box::new(
                        TyChirho::VarChirho(in_a_chirho),
                    )))),
                ),
            },
        );
    }

    // ── Library function type schemes (imported modules) ────────────────

    // coerce :: Coercible a b => a -> b
    {
        let a_chirho = TyVarChirho(7100);
        let b_chirho = TyVarChirho(7101);
        env_chirho.bind_chirho(
            "coerce".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::VarChirho(b_chirho),
                ),
            },
        );
    }

    // unsafeCoerce :: a -> b
    {
        let a_chirho = TyVarChirho(7110);
        let b_chirho = TyVarChirho(7111);
        let scheme_chirho = SchemeChirho {
            vars_chirho: vec![a_chirho, b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::VarChirho(a_chirho),
                TyChirho::VarChirho(b_chirho),
            ),
        };
        env_chirho.bind_chirho("unsafeCoerce".to_string(), scheme_chirho.clone());
        env_chirho.bind_chirho("unsafeCoerce#".to_string(), scheme_chirho);
    }

    {
        let int_hash_ty_chirho = TyChirho::ConChirho("Int#".to_string());
        let ishift_scheme_chirho = SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
            vec![int_hash_ty_chirho.clone(), int_hash_ty_chirho.clone()],
            int_hash_ty_chirho,
        ));
        for name_chirho in [
            "iShiftL#",
            "iShiftRA#",
            "iShiftRL#",
            "GHC.Exts.iShiftL#",
            "GHC.Exts.iShiftRA#",
            "GHC.Exts.iShiftRL#",
            "GHC.Prim.iShiftL#",
            "GHC.Prim.iShiftRA#",
            "GHC.Prim.iShiftRL#",
        ] {
            env_chirho.bind_chirho(name_chirho.to_string(), ishift_scheme_chirho.clone());
        }
    }

    {
        let unchecked_shift_a_chirho = TyVarChirho(7112);
        let unchecked_shift_scheme_chirho = SchemeChirho {
            vars_chirho: vec![unchecked_shift_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(unchecked_shift_a_chirho),
                    TyChirho::ConChirho("Int#".to_string()),
                ],
                TyChirho::VarChirho(unchecked_shift_a_chirho),
            ),
        };
        for name_chirho in [
            "uncheckedShiftL#",
            "uncheckedShiftRL#",
            "GHC.Exts.uncheckedShiftL#",
            "GHC.Exts.uncheckedShiftRL#",
            "GHC.Prim.uncheckedShiftL#",
            "GHC.Prim.uncheckedShiftRL#",
        ] {
            env_chirho.bind_chirho(
                name_chirho.to_string(),
                unchecked_shift_scheme_chirho.clone(),
            );
        }
    }

    // keepAlive# :: forall a s b. a -> State# s -> (State# s -> b) -> b
    {
        let keep_alive_a_chirho = TyVarChirho(7113);
        let keep_alive_s_chirho = TyVarChirho(7114);
        let keep_alive_b_chirho = TyVarChirho(7115);
        let state_s_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("State#".to_string())),
            Box::new(TyChirho::VarChirho(keep_alive_s_chirho)),
        );
        let keep_alive_scheme_chirho = SchemeChirho {
            vars_chirho: vec![
                keep_alive_a_chirho,
                keep_alive_s_chirho,
                keep_alive_b_chirho,
            ],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::VarChirho(keep_alive_a_chirho),
                    state_s_chirho.clone(),
                    TyChirho::fun_chirho(
                        state_s_chirho.clone(),
                        TyChirho::VarChirho(keep_alive_b_chirho),
                    ),
                ],
                TyChirho::VarChirho(keep_alive_b_chirho),
            ),
        };
        for name_chirho in ["keepAlive#", "GHC.Exts.keepAlive#", "GHC.Prim.keepAlive#"] {
            env_chirho.bind_chirho(name_chirho.to_string(), keep_alive_scheme_chirho.clone());
        }
    }

    // setByteArray# :: forall s. MutableByteArray# s -> Int# -> Int# -> Int# -> State# s -> State# s
    {
        let s_chirho = TyVarChirho(7116);
        let mutable_byte_array_s_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("MutableByteArray#".to_string())),
            Box::new(TyChirho::VarChirho(s_chirho)),
        );
        let state_s_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::ConChirho("State#".to_string())),
            Box::new(TyChirho::VarChirho(s_chirho)),
        );
        env_chirho.bind_chirho(
            "setByteArray#".to_string(),
            SchemeChirho {
                vars_chirho: vec![s_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        mutable_byte_array_s_chirho,
                        TyChirho::ConChirho("Int#".to_string()),
                        TyChirho::ConChirho("Int#".to_string()),
                        TyChirho::ConChirho("Int#".to_string()),
                        state_s_chirho.clone(),
                    ],
                    state_s_chirho,
                ),
            },
        );
    }

    // GHC.Integer.Logarithms / Compat
    {
        let integer_ty_chirho = TyChirho::ConChirho("Integer".to_string());
        let int_hash_ty_chirho = TyChirho::ConChirho("Int#".to_string());
        let word_hash_ty_chirho = TyChirho::ConChirho("Word#".to_string());

        env_chirho.bind_chirho(
            "integerLog2#".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                integer_ty_chirho.clone(),
                int_hash_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            "integerLogBase#".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_n_chirho(
                vec![integer_ty_chirho.clone(), integer_ty_chirho],
                int_hash_ty_chirho.clone(),
            )),
        );
        env_chirho.bind_chirho(
            "wordLog2#".to_string(),
            SchemeChirho::mono_chirho(TyChirho::fun_chirho(
                word_hash_ty_chirho,
                int_hash_ty_chirho,
            )),
        );
    }

    // cast :: (Typeable a, Typeable b) => a -> Maybe b
    {
        let a_chirho = TyVarChirho(7120);
        let b_chirho = TyVarChirho(7121);
        env_chirho.bind_chirho(
            "cast".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(TyChirho::VarChirho(b_chirho)),
                    ),
                ),
            },
        );
    }

    // eqT :: (Typeable a, Typeable b) => Maybe (a :~: b)
    // Simplified: eqT :: Maybe a (since :~: is complex)
    {
        let a_chirho = TyVarChirho(7130);
        env_chirho.bind_chirho(
            "eqT".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // typeOf :: Typeable a => a -> TypeRep a
    {
        let a_chirho = TyVarChirho(7140);
        env_chirho.bind_chirho(
            "typeOf".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::VarChirho(a_chirho),
                    mk_type_rep_ty_chirho(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // typeRepFingerprint :: TypeRep a -> Fingerprint
    {
        let type_rep_fingerprint_a_chirho = TyVarChirho(7141);
        let type_rep_fingerprint_scheme_chirho = SchemeChirho {
            vars_chirho: vec![type_rep_fingerprint_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_fingerprint_a_chirho)),
                TyChirho::ConChirho("Fingerprint".to_string()),
            ),
        };
        env_chirho.bind_chirho(
            "typeRepFingerprint".to_string(),
            type_rep_fingerprint_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Type.Reflection.typeRepFingerprint".to_string(),
            type_rep_fingerprint_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Type.Reflection.Unsafe.typeRepFingerprint".to_string(),
            type_rep_fingerprint_scheme_chirho,
        );
    }

    // typeRep variants:
    // Data.Typeable exports the older proxy-argument form, while Type.Reflection
    // exports the modern visible-type-application form.
    {
        let a_chirho = TyVarChirho(7150);
        let type_reflection_rep_scheme_chirho = SchemeChirho {
            vars_chirho: vec![a_chirho],
            preds_chirho: vec![],
            ty_chirho: mk_type_rep_ty_chirho(TyChirho::VarChirho(a_chirho)),
        };
        let proxy_rep_a_chirho = TyVarChirho(7151);
        let data_typeable_rep_scheme_chirho = SchemeChirho {
            vars_chirho: vec![proxy_rep_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                    Box::new(TyChirho::VarChirho(proxy_rep_a_chirho)),
                ),
                mk_type_rep_ty_chirho(TyChirho::VarChirho(proxy_rep_a_chirho)),
            ),
        };
        env_chirho.bind_chirho(
            "typeRep".to_string(),
            data_typeable_rep_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Type.Reflection.typeRep".to_string(),
            type_reflection_rep_scheme_chirho,
        );
        env_chirho.bind_chirho(
            "Data.Typeable.typeRep".to_string(),
            data_typeable_rep_scheme_chirho,
        );
    }

    // Proxy :: Proxy a (constructor)
    {
        let a_chirho = TyVarChirho(7160);
        env_chirho.bind_chirho(
            "Proxy".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::ConChirho("Proxy".to_string())),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // eqTypeRep :: TypeRep a -> TypeRep b -> Maybe (a :~~: b)
    {
        let left_a_chirho = TyVarChirho(7161);
        let right_b_chirho = TyVarChirho(7162);
        let proof_ty_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::ConChirho(":~~:".to_string())),
                Box::new(TyChirho::VarChirho(left_a_chirho)),
            )),
            Box::new(TyChirho::VarChirho(right_b_chirho)),
        );
        let eq_type_rep_scheme_chirho = SchemeChirho {
            vars_chirho: vec![left_a_chirho, right_b_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                mk_type_rep_ty_chirho(TyChirho::VarChirho(left_a_chirho)),
                TyChirho::fun_chirho(
                    mk_type_rep_ty_chirho(TyChirho::VarChirho(right_b_chirho)),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("Maybe".to_string())),
                        Box::new(proof_ty_chirho),
                    ),
                ),
            ),
        };
        env_chirho.bind_chirho("eqTypeRep".to_string(), eq_type_rep_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Type.Reflection.eqTypeRep".to_string(),
            eq_type_rep_scheme_chirho,
        );
    }

    // typeRepTyCon :: TypeRep a -> TyCon
    {
        let type_rep_a_chirho = TyVarChirho(7163);
        let type_rep_ty_con_scheme_chirho = SchemeChirho {
            vars_chirho: vec![type_rep_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_a_chirho)),
                TyChirho::ConChirho("TyCon".to_string()),
            ),
        };
        env_chirho.bind_chirho(
            "typeRepTyCon".to_string(),
            type_rep_ty_con_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Type.Reflection.typeRepTyCon".to_string(),
            type_rep_ty_con_scheme_chirho,
        );
    }

    // typeRepKind :: TypeRep a -> TypeRep k
    {
        let type_rep_a_chirho = TyVarChirho(7164);
        let type_rep_k_chirho = TyVarChirho(7165);
        let type_rep_kind_scheme_chirho = SchemeChirho {
            vars_chirho: vec![type_rep_a_chirho, type_rep_k_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_a_chirho)),
                mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_k_chirho)),
            ),
        };
        env_chirho.bind_chirho(
            "typeRepKind".to_string(),
            type_rep_kind_scheme_chirho.clone(),
        );
        env_chirho.bind_chirho(
            "Type.Reflection.typeRepKind".to_string(),
            type_rep_kind_scheme_chirho,
        );
    }

    // mkTrCon :: TyCon -> [SomeTypeRep] -> TypeRep a
    {
        let type_rep_a_chirho = TyVarChirho(7166);
        let mk_tr_con_scheme_chirho = SchemeChirho {
            vars_chirho: vec![type_rep_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("TyCon".to_string()),
                TyChirho::fun_chirho(
                    TyChirho::ListChirho(Box::new(TyChirho::ConChirho("SomeTypeRep".to_string()))),
                    mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_a_chirho)),
                ),
            ),
        };
        env_chirho.bind_chirho("mkTrCon".to_string(), mk_tr_con_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Type.Reflection.Unsafe.mkTrCon".to_string(),
            mk_tr_con_scheme_chirho,
        );
    }

    // mkTrApp :: TypeRep f -> TypeRep x -> TypeRep a
    {
        let type_rep_f_chirho = TyVarChirho(7167);
        let type_rep_x_chirho = TyVarChirho(7168);
        let type_rep_a_chirho = TyVarChirho(7169);
        let mk_tr_app_scheme_chirho = SchemeChirho {
            vars_chirho: vec![type_rep_f_chirho, type_rep_x_chirho, type_rep_a_chirho],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_f_chirho)),
                TyChirho::fun_chirho(
                    mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_x_chirho)),
                    mk_type_rep_ty_chirho(TyChirho::VarChirho(type_rep_a_chirho)),
                ),
            ),
        };
        env_chirho.bind_chirho("mkTrApp".to_string(), mk_tr_app_scheme_chirho.clone());
        env_chirho.bind_chirho(
            "Type.Reflection.Unsafe.mkTrApp".to_string(),
            mk_tr_app_scheme_chirho,
        );
    }

    // TyCon record-style accessors and constructor.
    {
        let ty_con_text_scheme_chirho = SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("TyCon".to_string()),
                TyChirho::string_chirho(),
            ),
        };
        for name_chirho in ["tyConPackage", "tyConModule", "tyConName"] {
            env_chirho.bind_chirho(name_chirho.to_string(), ty_con_text_scheme_chirho.clone());
        }

        let ty_con_kind_args_scheme_chirho = SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("TyCon".to_string()),
                TyChirho::ListChirho(Box::new(TyChirho::ConChirho("SomeTypeRep".to_string()))),
            ),
        };
        env_chirho.bind_chirho("tyConKindArgs".to_string(), ty_con_kind_args_scheme_chirho);

        let ty_con_kind_rep_scheme_chirho = SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_chirho(
                TyChirho::ConChirho("TyCon".to_string()),
                TyChirho::ConChirho("KindRep".to_string()),
            ),
        };
        env_chirho.bind_chirho("tyConKindRep".to_string(), ty_con_kind_rep_scheme_chirho);

        let mk_ty_con_scheme_chirho = SchemeChirho {
            vars_chirho: vec![],
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                vec![
                    TyChirho::string_chirho(),
                    TyChirho::string_chirho(),
                    TyChirho::string_chirho(),
                    TyChirho::ListChirho(Box::new(TyChirho::ConChirho("SomeTypeRep".to_string()))),
                    TyChirho::ConChirho("KindRep".to_string()),
                ],
                TyChirho::ConChirho("TyCon".to_string()),
            ),
        };
        env_chirho.bind_chirho("mkTyCon".to_string(), mk_ty_con_scheme_chirho);
    }

    // oneShot :: (a -> b) -> a -> b
    {
        let a_chirho = TyVarChirho(7170);
        let b_chirho = TyVarChirho(7171);
        env_chirho.bind_chirho(
            "oneShot".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(b_chirho),
                    ),
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(b_chirho),
                    ),
                ),
            },
        );
    }

    // comparing :: Ord b => (a -> b) -> a -> a -> Ordering
    {
        let a_chirho = TyVarChirho(7180);
        let b_chirho = TyVarChirho(7181);
        env_chirho.bind_chirho(
            "comparing".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Ord".to_string(),
                    ty_chirho: TyChirho::VarChirho(b_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(b_chirho),
                        ),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(a_chirho),
                    ],
                    TyChirho::ConChirho("Ordering".to_string()),
                ),
            },
        );
    }

    // liftIO :: MonadIO m => IO a -> m a
    {
        let m_chirho = TyVarChirho(7190);
        let a_chirho = TyVarChirho(7191);
        env_chirho.bind_chirho(
            "liftIO".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::ConChirho("IO".to_string())),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
    }

    // on :: (b -> b -> c) -> (a -> b) -> a -> a -> c
    {
        let a_chirho = TyVarChirho(7200);
        let b_chirho = TyVarChirho(7201);
        let c_chirho = TyVarChirho(7202);
        env_chirho.bind_chirho(
            "on".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_n_chirho(
                            vec![TyChirho::VarChirho(b_chirho), TyChirho::VarChirho(b_chirho)],
                            TyChirho::VarChirho(c_chirho),
                        ),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(b_chirho),
                        ),
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(a_chirho),
                    ],
                    TyChirho::VarChirho(c_chirho),
                ),
            },
        );
    }

    // fix :: (a -> a) -> a
    {
        let a_chirho = TyVarChirho(7210);
        env_chirho.bind_chirho(
            "fix".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::fun_chirho(
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::VarChirho(a_chirho),
                    ),
                    TyChirho::VarChirho(a_chirho),
                ),
            },
        );
    }

    // void :: Functor f => f a -> f ()
    {
        let f_chirho = TyVarChirho(7220);
        let a_chirho = TyVarChirho(7221);
        env_chirho.bind_chirho(
            "void".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Functor".to_string(),
                    ty_chirho: TyChirho::VarChirho(f_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // when :: Applicative f => Bool -> f () -> f ()
    {
        let f_chirho = TyVarChirho(7230);
        env_chirho.bind_chirho(
            "when".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::bool_chirho(),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::TupleChirho(vec![])),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // unless :: Applicative f => Bool -> f () -> f ()
    {
        let f_chirho = TyVarChirho(7240);
        env_chirho.bind_chirho(
            "unless".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::bool_chirho(),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::TupleChirho(vec![])),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // join :: Monad m => m (m a) -> m a
    {
        let m_chirho = TyVarChirho(7250);
        let a_chirho = TyVarChirho(7251);
        env_chirho.bind_chirho(
            "join".to_string(),
            SchemeChirho {
                vars_chirho: vec![m_chirho, a_chirho],
                preds_chirho: vec![SchemePredChirho {
                    class_name_chirho: "Monad".to_string(),
                    ty_chirho: TyChirho::VarChirho(m_chirho),
                    extra_tys_chirho: vec![],
                }],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(m_chirho)),
                            Box::new(TyChirho::VarChirho(a_chirho)),
                        )),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(m_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                ),
            },
        );
    }

    // guard :: Alternative f => Bool -> f ()
    {
        let f_chirho = TyVarChirho(7260);
        env_chirho.bind_chirho(
            "guard".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::bool_chirho(),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::TupleChirho(vec![])),
                    ),
                ),
            },
        );
    }

    // empty :: f a
    {
        let f_chirho = TyVarChirho(7261);
        let a_chirho = TyVarChirho(7262);
        env_chirho.bind_chirho(
            "empty".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(f_chirho)),
                    Box::new(TyChirho::VarChirho(a_chirho)),
                ),
            },
        );
    }

    // (<|>) :: f a -> f a -> f a
    {
        let f_chirho = TyVarChirho(7263);
        let a_chirho = TyVarChirho(7264);
        let fa_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(f_chirho)),
            Box::new(TyChirho::VarChirho(a_chirho)),
        );
        env_chirho.bind_chirho(
            "<|>".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![fa_chirho.clone(), fa_chirho.clone()],
                    fa_chirho,
                ),
            },
        );
    }

    // some :: f a -> f [a]
    {
        let f_chirho = TyVarChirho(7265);
        let a_chirho = TyVarChirho(7266);
        env_chirho.bind_chirho(
            "some".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                            a_chirho,
                        )))),
                    ),
                ),
            },
        );
    }

    // many :: f a -> f [a]
    {
        let f_chirho = TyVarChirho(7267);
        let a_chirho = TyVarChirho(7268);
        env_chirho.bind_chirho(
            "many".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                            a_chirho,
                        )))),
                    ),
                ),
            },
        );
    }

    // optional :: f a -> f (Maybe a)
    {
        let f_chirho = TyVarChirho(7269);
        let a_chirho = TyVarChirho(7270);
        env_chirho.bind_chirho(
            "optional".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Maybe".to_string())),
                            Box::new(TyChirho::VarChirho(a_chirho)),
                        )),
                    ),
                ),
            },
        );
    }

    // forever :: Applicative f => f a -> f b
    {
        let f_chirho = TyVarChirho(7271);
        let a_chirho = TyVarChirho(7272);
        let b_chirho = TyVarChirho(7273);
        env_chirho.bind_chirho(
            "forever".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_chirho(
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(a_chirho)),
                    ),
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(b_chirho)),
                    ),
                ),
            },
        );
    }

    // liftA2 :: (a -> b -> c) -> f a -> f b -> f c
    {
        let f_chirho = TyVarChirho(7274);
        let a_chirho = TyVarChirho(7275);
        let b_chirho = TyVarChirho(7276);
        let c_chirho = TyVarChirho(7277);
        env_chirho.bind_chirho(
            "liftA2".to_string(),
            SchemeChirho {
                vars_chirho: vec![f_chirho, a_chirho, b_chirho, c_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(b_chirho),
                                TyChirho::VarChirho(c_chirho),
                            ),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::VarChirho(a_chirho)),
                        ),
                        TyChirho::AppChirho(
                            Box::new(TyChirho::VarChirho(f_chirho)),
                            Box::new(TyChirho::VarChirho(b_chirho)),
                        ),
                    ],
                    TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(f_chirho)),
                        Box::new(TyChirho::VarChirho(c_chirho)),
                    ),
                ),
            },
        );
    }

    // Data.Function: (&) :: a -> (a -> b) -> b
    {
        let a_chirho = TyVarChirho(7280);
        let b_chirho = TyVarChirho(7281);
        env_chirho.bind_chirho(
            "&".to_string(),
            SchemeChirho {
                vars_chirho: vec![a_chirho, b_chirho],
                preds_chirho: vec![],
                ty_chirho: TyChirho::fun_n_chirho(
                    vec![
                        TyChirho::VarChirho(a_chirho),
                        TyChirho::fun_chirho(
                            TyChirho::VarChirho(a_chirho),
                            TyChirho::VarChirho(b_chirho),
                        ),
                    ],
                    TyChirho::VarChirho(b_chirho),
                ),
            },
        );
    }
}
