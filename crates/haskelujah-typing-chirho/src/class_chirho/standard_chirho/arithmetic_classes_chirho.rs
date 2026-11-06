// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Contiguous standard-seed segment; order is owned by the parent dispatcher.

use std::collections::HashMap;

use crate::ty_chirho::{SchemeChirho, TyChirho, TyVarChirho};

use super::super::ClassEnvChirho;

impl ClassEnvChirho {
    pub(super) fn seed_arithmetic_classes_chirho(&mut self) {
        // Eq: `==` and `/=`, each defaulting to the other's negation in the dictionary
        // pass, as GHC's do (row 606).
        let eq_var_chirho = TyVarChirho(9000);
        let eq_method_chirho = SchemeChirho {
            vars_chirho: vec![eq_var_chirho].into(),
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(eq_var_chirho),
                    TyChirho::VarChirho(eq_var_chirho),
                ],
                TyChirho::bool_chirho(),
            ),
        };
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "Eq".to_string(),
                supers_chirho: vec![],
                var_chirho: eq_var_chirho,
                methods_chirho: HashMap::from([
                    ("==".to_string(), eq_method_chirho.clone()),
                    ("/=".to_string(), eq_method_chirho),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Ord (superclass: Eq): GHC's seven methods. `compare` is the minimal definition
        // here, and the dictionary pass defaults the other six from it (row 612).
        let ord_var_chirho = TyVarChirho(9001);
        let ord_method_chirho = |result_ty_chirho: TyChirho| SchemeChirho {
            vars_chirho: vec![ord_var_chirho].into(),
            preds_chirho: vec![],
            ty_chirho: TyChirho::fun_n_chirho(
                [
                    TyChirho::VarChirho(ord_var_chirho),
                    TyChirho::VarChirho(ord_var_chirho),
                ],
                result_ty_chirho,
            ),
        };
        let ord_methods_chirho = ["<", "<=", ">", ">="]
            .map(|name_chirho| (name_chirho, TyChirho::bool_chirho()))
            .into_iter()
            .chain([("compare", TyChirho::ConChirho("Ordering".to_string()))])
            .chain(
                ["max", "min"]
                    .map(|name_chirho| (name_chirho, TyChirho::VarChirho(ord_var_chirho))),
            )
            .map(|(name_chirho, result_ty_chirho)| {
                (name_chirho.to_string(), ord_method_chirho(result_ty_chirho))
            })
            .collect();
        self.add_seed_class_with_superclasses_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho,
            &[haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho],
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "Ord".to_string(),
                supers_chirho: Vec::new(),
                var_chirho: ord_var_chirho,
                methods_chirho: ord_methods_chirho,
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Show
        let show_var_chirho = TyVarChirho(9002);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "Show".to_string(),
                supers_chirho: vec![],
                var_chirho: show_var_chirho,
                methods_chirho: HashMap::from([(
                    "show".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![show_var_chirho].into(),
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::VarChirho(show_var_chirho),
                            TyChirho::string_chirho(),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Num
        let num_var_chirho = TyVarChirho(9003);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NumChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "Num".to_string(),
                // GHC's Num has no superclasses (Eq and Show were dropped in GHC 7.4);
                // the Haskell 98 pair made `Num a` entail `Eq a` and gave every Num
                // dictionary two phantom superclass slots.
                supers_chirho: vec![],
                var_chirho: num_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "+".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(num_var_chirho),
                                    TyChirho::VarChirho(num_var_chirho),
                                ],
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                    (
                        "*".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(num_var_chirho),
                                    TyChirho::VarChirho(num_var_chirho),
                                ],
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                    (
                        "-".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(num_var_chirho),
                                    TyChirho::VarChirho(num_var_chirho),
                                ],
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                    (
                        "negate".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(num_var_chirho),
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                    (
                        "fromInteger".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::ConChirho("Integer".to_string()),
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                    (
                        "abs".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(num_var_chirho),
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                    (
                        "signum".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![num_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(num_var_chirho),
                                TyChirho::VarChirho(num_var_chirho),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Fractional (superclass: Num)
        let frac_var_chirho = TyVarChirho(9010);
        self.add_seed_class_with_superclasses_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FractionalChirho,
            &[haskelujah_ast_chirho::name_chirho::CompilerClassChirho::NumChirho],
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "Fractional".to_string(),
                supers_chirho: Vec::new(),
                var_chirho: frac_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "/".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![frac_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(frac_var_chirho),
                                    TyChirho::VarChirho(frac_var_chirho),
                                ],
                                TyChirho::VarChirho(frac_var_chirho),
                            ),
                        },
                    ),
                    (
                        "recip".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![frac_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(frac_var_chirho),
                                TyChirho::VarChirho(frac_var_chirho),
                            ),
                        },
                    ),
                    (
                        "fromRational".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![frac_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::ConChirho("Rational".to_string()),
                                TyChirho::VarChirho(frac_var_chirho),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Bits
        let bits_var_chirho = TyVarChirho(9012);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BitsChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "Bits".to_string(),
                supers_chirho: vec![],
                var_chirho: bits_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        ".&.".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(bits_var_chirho),
                                    TyChirho::VarChirho(bits_var_chirho),
                                ],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        ".|.".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(bits_var_chirho),
                                    TyChirho::VarChirho(bits_var_chirho),
                                ],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "xor".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(bits_var_chirho),
                                    TyChirho::VarChirho(bits_var_chirho),
                                ],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "complement".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(bits_var_chirho),
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "shift".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [TyChirho::VarChirho(bits_var_chirho), TyChirho::int_chirho()],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "shiftL".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [TyChirho::VarChirho(bits_var_chirho), TyChirho::int_chirho()],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "shiftR".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [TyChirho::VarChirho(bits_var_chirho), TyChirho::int_chirho()],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "bit".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::int_chirho(),
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "setBit".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [TyChirho::VarChirho(bits_var_chirho), TyChirho::int_chirho()],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "clearBit".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [TyChirho::VarChirho(bits_var_chirho), TyChirho::int_chirho()],
                                TyChirho::VarChirho(bits_var_chirho),
                            ),
                        },
                    ),
                    (
                        "testBit".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [TyChirho::VarChirho(bits_var_chirho), TyChirho::int_chirho()],
                                TyChirho::bool_chirho(),
                            ),
                        },
                    ),
                    (
                        "bitSize".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(bits_var_chirho),
                                TyChirho::int_chirho(),
                            ),
                        },
                    ),
                    (
                        "isSigned".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(bits_var_chirho),
                                TyChirho::bool_chirho(),
                            ),
                        },
                    ),
                    (
                        "popCount".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(bits_var_chirho),
                                TyChirho::int_chirho(),
                            ),
                        },
                    ),
                    (
                        "zeroBits".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::VarChirho(bits_var_chirho),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // FiniteBits (superclass: Bits)
        let finite_bits_var_chirho = TyVarChirho(9013);
        self.add_seed_class_with_superclasses_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FiniteBitsChirho,
            &[haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BitsChirho],
            crate::class_chirho::ClassDeclPartsChirho {
                super_templates_chirho: Vec::new(),
                kind_vars_chirho: Vec::new(),
                name_chirho: "FiniteBits".to_string(),
                supers_chirho: Vec::new(),
                var_chirho: finite_bits_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "finiteBitSize".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![finite_bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(finite_bits_var_chirho),
                                TyChirho::int_chirho(),
                            ),
                        },
                    ),
                    (
                        "countLeadingZeros".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![finite_bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(finite_bits_var_chirho),
                                TyChirho::int_chirho(),
                            ),
                        },
                    ),
                    (
                        "countTrailingZeros".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![finite_bits_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(finite_bits_var_chirho),
                                TyChirho::int_chirho(),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );
    }
}
