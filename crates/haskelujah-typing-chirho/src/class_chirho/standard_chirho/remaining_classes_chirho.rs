// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Contiguous standard-seed segment; order is owned by the parent dispatcher.

use std::collections::HashMap;

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use super::super::ClassEnvChirho;

impl ClassEnvChirho {
    pub(super) fn seed_remaining_classes_chirho(&mut self) {
        // Floating (superclass: Fractional)
        let floating_var_chirho = TyVarChirho(9009);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FloatingChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Floating".to_string(),
                supers_chirho: vec!["Fractional".to_string()],
                var_chirho: floating_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "pi".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::VarChirho(floating_var_chirho),
                        },
                    ),
                    (
                        "exp".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "log".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "sqrt".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "sin".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "cos".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "tan".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "asin".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "acos".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                    (
                        "atan".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![floating_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(floating_var_chirho),
                                TyChirho::VarChirho(floating_var_chirho),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance Floating Double
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FloatingChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Floating".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Double".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Floating Float
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FloatingChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Floating".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Float".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // RealFrac (superclass: Real, Fractional)
        let realfrac_var_chirho = TyVarChirho(9060);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFracChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "RealFrac".to_string(),
                supers_chirho: vec!["Real".to_string(), "Fractional".to_string()],
                var_chirho: realfrac_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance RealFrac Double / Float
        for ty_name_chirho in &["Double", "Float"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFracChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "RealFrac".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // RealFloat (superclass: RealFrac, Floating)
        let realfloat_var_chirho = TyVarChirho(9061);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFloatChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "RealFloat".to_string(),
                supers_chirho: vec!["RealFrac".to_string(), "Floating".to_string()],
                var_chirho: realfloat_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance RealFloat Double / Float
        for ty_name_chirho in &["Double", "Float"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealFloatChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "RealFloat".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // MonadIO class
        let monadio_var_chirho = TyVarChirho(9062);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadIoChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "MonadIO".to_string(),
                supers_chirho: vec!["Monad".to_string()],
                var_chirho: monadio_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance MonadIO IO
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadIoChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "MonadIO".to_string(),
                head_ty_chirho: TyChirho::ConChirho("IO".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // MonadFail class
        let monad_fail_m_chirho = TyVarChirho(9063);
        let monad_fail_a_chirho = TyVarChirho(9064);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadFailChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "MonadFail".to_string(),
                supers_chirho: vec!["Monad".to_string()],
                var_chirho: monad_fail_m_chirho,
                methods_chirho: HashMap::from([(
                    "fail".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![monad_fail_m_chirho, monad_fail_a_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::string_chirho(),
                            TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(monad_fail_m_chirho)),
                                Box::new(TyChirho::VarChirho(monad_fail_a_chirho)),
                            ),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance MonadFail IO / Maybe / []
        for ty_chirho in ["IO", "Maybe", "[]"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadFailChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "MonadFail".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // MonadFix class
        let monad_fix_m_chirho = TyVarChirho(9092);
        let monad_fix_a_chirho = TyVarChirho(9093);
        let monad_fix_result_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(monad_fix_m_chirho)),
            Box::new(TyChirho::VarChirho(monad_fix_a_chirho)),
        );
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadFixChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "MonadFix".to_string(),
                supers_chirho: vec!["Monad".to_string()],
                var_chirho: monad_fix_m_chirho,
                methods_chirho: HashMap::from([(
                    "mfix".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![monad_fix_m_chirho, monad_fix_a_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::fun_chirho(
                                TyChirho::VarChirho(monad_fix_a_chirho),
                                monad_fix_result_chirho.clone(),
                            ),
                            monad_fix_result_chirho,
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );
        for ty_chirho in ["IO", "Maybe", "[]"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadFixChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "MonadFix".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }
        let format_time_var_chirho = TyVarChirho(9065);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FormatTimeChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "FormatTime".to_string(),
                supers_chirho: vec![],
                var_chirho: format_time_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        let parse_time_var_chirho = TyVarChirho(9066);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ParseTimeChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "ParseTime".to_string(),
                supers_chirho: vec![],
                var_chirho: parse_time_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        for (class_name_chirho, class_identity_chirho) in [
            (
                "FormatTime",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FormatTimeChirho),
            ),
            (
                "ParseTime",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ParseTimeChirho),
            ),
        ] {
            for ty_name_chirho in [
                "Day",
                "TimeOfDay",
                "LocalTime",
                "ZonedTime",
                "TimeZone",
                "UTCTime",
                "UniversalTime",
                "NominalDiffTime",
                "DiffTime",
            ] {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_name_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }

        // Typeable — GHC built-in class. Every type is automatically Typeable.
        // We add a universal instance so that any Typeable constraint is satisfied.
        let typeable_var_chirho = TyVarChirho(9070);
        let typeable_kind_chirho = TyVarChirho(90700);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::TypeableChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: vec![typeable_kind_chirho],
                name_chirho: "Typeable".to_string(),
                supers_chirho: vec![],
                var_chirho: typeable_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );
        // Universal Typeable instance: every type is Typeable
        let typeable_inst_var_chirho = TyVarChirho(9071);
        let typeable_inst_kind_chirho = TyVarChirho(90710);
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::TypeableChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: vec![TyChirho::VarChirho(typeable_inst_kind_chirho)],
                class_name_chirho: "Typeable".to_string(),
                head_ty_chirho: TyChirho::VarChirho(typeable_inst_var_chirho),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Data class (Data.Data)
        let data_var_chirho = TyVarChirho(9072);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::DataChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Data".to_string(),
                supers_chirho: vec!["Typeable".to_string()],
                var_chirho: data_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // KnownNat / KnownSymbol / KnownChar — GHC type-level classes
        for (class_name_chirho, class_identity_chirho) in [
            (
                "KnownNat",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho),
            ),
            (
                "KnownSymbol",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownSymbolChirho),
            ),
            (
                "KnownChar",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownCharChirho),
            ),
        ] {
            let kn_var_chirho = TyVarChirho(9073);
            self.add_seed_class_chirho(
                class_identity_chirho
                    .expect("this compiler class declaration has an explicit seed identity"),
                crate::class_chirho::ClassDeclPartsChirho {
                    kind_vars_chirho: Vec::new(),
                    name_chirho: class_name_chirho.to_string(),
                    supers_chirho: vec![],
                    var_chirho: kn_var_chirho,
                    methods_chirho: HashMap::new(),
                    extra_vars_chirho: vec![],
                    fundeps_chirho: vec![],
                    defaults_chirho: HashMap::new(),
                },
            );
        }

        // MonadZip (superclass: Monad)
        let monadzip_var_chirho = TyVarChirho(9074);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadZipChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "MonadZip".to_string(),
                supers_chirho: vec!["Monad".to_string()],
                var_chirho: monadzip_var_chirho,
                methods_chirho: HashMap::from([(
                    "mzipWith".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![
                            monadzip_var_chirho,
                            TyVarChirho(9075),
                            TyVarChirho(9076),
                            TyVarChirho(9077),
                        ],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::fun_chirho(
                                    TyChirho::VarChirho(TyVarChirho(9075)),
                                    TyChirho::fun_chirho(
                                        TyChirho::VarChirho(TyVarChirho(9076)),
                                        TyChirho::VarChirho(TyVarChirho(9077)),
                                    ),
                                ),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(monadzip_var_chirho)),
                                    Box::new(TyChirho::VarChirho(TyVarChirho(9075))),
                                ),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(monadzip_var_chirho)),
                                    Box::new(TyChirho::VarChirho(TyVarChirho(9076))),
                                ),
                            ],
                            TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(monadzip_var_chirho)),
                                Box::new(TyChirho::VarChirho(TyVarChirho(9077))),
                            ),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // RandomGen
        let randomgen_var_chirho = TyVarChirho(90741);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RandomGenChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "RandomGen".to_string(),
                supers_chirho: vec![],
                var_chirho: randomgen_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "split".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![randomgen_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(randomgen_var_chirho),
                                TyChirho::TupleChirho(vec![
                                    TyChirho::VarChirho(randomgen_var_chirho),
                                    TyChirho::VarChirho(randomgen_var_chirho),
                                ]),
                            ),
                        },
                    ),
                    (
                        "genRange".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![randomgen_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(randomgen_var_chirho),
                                TyChirho::TupleChirho(vec![
                                    TyChirho::int_chirho(),
                                    TyChirho::int_chirho(),
                                ]),
                            ),
                        },
                    ),
                    (
                        "next".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![randomgen_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(randomgen_var_chirho),
                                TyChirho::TupleChirho(vec![
                                    TyChirho::int_chirho(),
                                    TyChirho::VarChirho(randomgen_var_chirho),
                                ]),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // SplitGen (superclass: RandomGen)
        let splitgen_var_chirho = TyVarChirho(90742);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::SplitGenChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "SplitGen".to_string(),
                supers_chirho: vec!["RandomGen".to_string()],
                var_chirho: splitgen_var_chirho,
                methods_chirho: HashMap::from([(
                    "splitGen".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![splitgen_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(splitgen_var_chirho),
                            TyChirho::TupleChirho(vec![
                                TyChirho::VarChirho(splitgen_var_chirho),
                                TyChirho::VarChirho(splitgen_var_chirho),
                            ]),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Contravariant
        let contravariant_var_chirho = TyVarChirho(9078);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ContravariantChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Contravariant".to_string(),
                supers_chirho: vec![],
                var_chirho: contravariant_var_chirho,
                methods_chirho: HashMap::from([(
                    "contramap".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![
                            contravariant_var_chirho,
                            TyVarChirho(9079),
                            TyVarChirho(9080),
                        ],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            vec![
                                TyChirho::fun_chirho(
                                    TyChirho::VarChirho(TyVarChirho(9079)),
                                    TyChirho::VarChirho(TyVarChirho(9080)),
                                ),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(contravariant_var_chirho)),
                                    Box::new(TyChirho::VarChirho(TyVarChirho(9080))),
                                ),
                            ],
                            TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(contravariant_var_chirho)),
                                Box::new(TyChirho::VarChirho(TyVarChirho(9079))),
                            ),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Coercible — GHC built-in class for safe coercions between
        // types with the same representation. We treat it as a two-parameter
        // class with a universal instance: Coercible a a (reflexivity).
        // Real Coercible resolution requires role analysis; for now we
        // accept any Coercible constraint to avoid false E0204 errors.
        let coercible_var_a_chirho = TyVarChirho(9064);
        let coercible_var_b_chirho = TyVarChirho(9065);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::CoercibleChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Coercible".to_string(),
                supers_chirho: vec![],
                var_chirho: coercible_var_a_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![coercible_var_b_chirho],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Universal Coercible instance: Coercible a a (reflexivity)
        let coercible_inst_var_chirho = TyVarChirho(9066);
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::CoercibleChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Coercible".to_string(),
                head_ty_chirho: TyChirho::VarChirho(coercible_inst_var_chirho),
                extra_head_tys_chirho: vec![TyChirho::VarChirho(coercible_inst_var_chirho)],
                context_chirho: vec![],
            },
        );

        // Real (superclass: Num, Ord)
        let real_var_chirho = TyVarChirho(9063);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Real".to_string(),
                supers_chirho: vec!["Num".to_string(), "Ord".to_string()],
                var_chirho: real_var_chirho,
                methods_chirho: HashMap::new(),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance Real Int / Integer / Word / Double / Float (every Integral
        // type is Real: Integral's superclass dictionary needs it)
        for ty_name_chirho in &["Int", "Integer", "Word", "Double", "Float"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::RealChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Real".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Integral: GHC's `class (Real a, Enum a) => Integral a`. `==` and `<`
        // under an `Integral a` context come through Real => Ord => Eq, not
        // through Num, which has no superclasses.
        let integral_var_chirho = TyVarChirho(9008);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IntegralChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Integral".to_string(),
                supers_chirho: vec!["Real".to_string(), "Enum".to_string()],
                var_chirho: integral_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "div".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![integral_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(integral_var_chirho),
                                    TyChirho::VarChirho(integral_var_chirho),
                                ],
                                TyChirho::VarChirho(integral_var_chirho),
                            ),
                        },
                    ),
                    (
                        "mod".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![integral_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(integral_var_chirho),
                                    TyChirho::VarChirho(integral_var_chirho),
                                ],
                                TyChirho::VarChirho(integral_var_chirho),
                            ),
                        },
                    ),
                    (
                        "quot".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![integral_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(integral_var_chirho),
                                    TyChirho::VarChirho(integral_var_chirho),
                                ],
                                TyChirho::VarChirho(integral_var_chirho),
                            ),
                        },
                    ),
                    (
                        "rem".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![integral_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(integral_var_chirho),
                                    TyChirho::VarChirho(integral_var_chirho),
                                ],
                                TyChirho::VarChirho(integral_var_chirho),
                            ),
                        },
                    ),
                    (
                        "toInteger".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![integral_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(integral_var_chirho),
                                TyChirho::ConChirho("Integer".to_string()),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // NFData
        let nfdata_var_chirho = TyVarChirho(9030);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NfDataChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "NFData".to_string(),
                supers_chirho: vec![],
                var_chirho: nfdata_var_chirho,
                methods_chirho: HashMap::from([(
                    "rnf".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![nfdata_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(nfdata_var_chirho),
                            TyChirho::TupleChirho(vec![]),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );
    }
}
