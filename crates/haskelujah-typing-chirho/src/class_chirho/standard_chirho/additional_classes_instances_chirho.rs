// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Contiguous standard-seed segment; order is owned by the parent dispatcher.

use std::collections::HashMap;

use crate::ty_chirho::{MultChirho, SchemeChirho, TyChirho, TyVarChirho};

use super::super::ClassEnvChirho;

impl ClassEnvChirho {
    pub(super) fn seed_additional_classes_instances_chirho(&mut self) {
        // ── Alternative / MonadPlus ──
        let alt_var_chirho = TyVarChirho(9055);
        let alt_a_chirho = TyVarChirho(9056);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::AlternativeChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Alternative".to_string(),
                supers_chirho: vec!["Applicative".to_string()],
                var_chirho: alt_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "empty".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![alt_var_chirho, alt_a_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(alt_var_chirho)),
                                Box::new(TyChirho::VarChirho(alt_a_chirho)),
                            ),
                        },
                    ),
                    (
                        "<|>".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![alt_var_chirho, alt_a_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(alt_var_chirho)),
                                        Box::new(TyChirho::VarChirho(alt_a_chirho)),
                                    ),
                                    TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(alt_var_chirho)),
                                        Box::new(TyChirho::VarChirho(alt_a_chirho)),
                                    ),
                                ],
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(alt_var_chirho)),
                                    Box::new(TyChirho::VarChirho(alt_a_chirho)),
                                ),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Alternative instances for [], Maybe, IO, STM, ReadP, ReadPrec
        for ty_chirho in ["[]", "Maybe", "IO", "STM", "ReadP", "ReadPrec"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::AlternativeChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Alternative".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // MonadPlus class (superclass: Monad, Alternative)
        let mp_var_chirho = TyVarChirho(9057);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadPlusChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "MonadPlus".to_string(),
                supers_chirho: vec!["Monad".to_string(), "Alternative".to_string()],
                var_chirho: mp_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "mzero".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![mp_var_chirho, alt_a_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::AppChirho(
                                Box::new(TyChirho::VarChirho(mp_var_chirho)),
                                Box::new(TyChirho::VarChirho(alt_a_chirho)),
                            ),
                        },
                    ),
                    (
                        "mplus".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![mp_var_chirho, alt_a_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(mp_var_chirho)),
                                        Box::new(TyChirho::VarChirho(alt_a_chirho)),
                                    ),
                                    TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(mp_var_chirho)),
                                        Box::new(TyChirho::VarChirho(alt_a_chirho)),
                                    ),
                                ],
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(mp_var_chirho)),
                                    Box::new(TyChirho::VarChirho(alt_a_chirho)),
                                ),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );
        for ty_chirho in ["[]", "Maybe", "IO", "STM", "ReadP", "ReadPrec"] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadPlusChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "MonadPlus".to_string(),
                    head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // ── Semigroup / Monoid ──

        // Semigroup (no superclass)
        let sg_var_chirho = TyVarChirho(9050);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::SemigroupChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Semigroup".to_string(),
                supers_chirho: vec![],
                var_chirho: sg_var_chirho,
                methods_chirho: HashMap::from([(
                    "<>".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![sg_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_n_chirho(
                            [
                                TyChirho::VarChirho(sg_var_chirho),
                                TyChirho::VarChirho(sg_var_chirho),
                            ],
                            TyChirho::VarChirho(sg_var_chirho),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Monoid (superclass: Semigroup)
        let mon_var_chirho = TyVarChirho(9051);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonoidChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Monoid".to_string(),
                supers_chirho: vec!["Semigroup".to_string()],
                var_chirho: mon_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "mappend".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![mon_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_n_chirho(
                                [
                                    TyChirho::VarChirho(mon_var_chirho),
                                    TyChirho::VarChirho(mon_var_chirho),
                                ],
                                TyChirho::VarChirho(mon_var_chirho),
                            ),
                        },
                    ),
                    (
                        "mempty".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![mon_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::VarChirho(mon_var_chirho),
                        },
                    ),
                    (
                        "mconcat".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![mon_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::ListChirho(Box::new(TyChirho::VarChirho(mon_var_chirho))),
                                TyChirho::VarChirho(mon_var_chirho),
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Semigroup ground instances for concrete list types
        for elem_chirho in &[
            TyChirho::ConChirho("Int".to_string()),
            TyChirho::ConChirho("Integer".to_string()),
            TyChirho::ConChirho("Char".to_string()),
            TyChirho::ConChirho("Double".to_string()),
            TyChirho::ConChirho("Bool".to_string()),
        ] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::SemigroupChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Semigroup".to_string(),
                    head_ty_chirho: TyChirho::ListChirho(Box::new(elem_chirho.clone())),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }
        // instance Semigroup ()
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::SemigroupChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Semigroup".to_string(),
                head_ty_chirho: TyChirho::TupleChirho(vec![]),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Unit derives these in base (`deriving instance Show ()` and friends
        // are wired in). They were missing here, so `print ()` looked like an
        // unsolved constraint to any check that trusts this table.
        for (class_chirho, class_identity_chirho) in [
            (
                "Show",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho),
            ),
            (
                "Read",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho),
            ),
            (
                "Eq",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho),
            ),
            (
                "Ord",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho),
            ),
            (
                "Bounded",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BoundedChirho),
            ),
            (
                "Enum",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EnumChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::TupleChirho(vec![]),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Monoid ground instances for concrete list types
        for elem_chirho in &[
            TyChirho::ConChirho("Int".to_string()),
            TyChirho::ConChirho("Integer".to_string()),
            TyChirho::ConChirho("Char".to_string()),
            TyChirho::ConChirho("Double".to_string()),
            TyChirho::ConChirho("Bool".to_string()),
        ] {
            self.add_seed_instance_chirho(
                haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonoidChirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: "Monoid".to_string(),
                    head_ty_chirho: TyChirho::ListChirho(Box::new(elem_chirho.clone())),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }
        // instance Monoid ()
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonoidChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "Monoid".to_string(),
                head_ty_chirho: TyChirho::TupleChirho(vec![]),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // instance Semigroup/Monoid Ordering
        for (class_chirho, class_identity_chirho) in [
            (
                "Semigroup",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::SemigroupChirho),
            ),
            (
                "Monoid",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonoidChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::ConChirho("Ordering".to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Foldable / Traversable instances for [], Maybe, Identity
        for (class_chirho, class_identity_chirho) in [
            (
                "Foldable",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FoldableChirho),
            ),
            (
                "Traversable",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::TraversableChirho),
            ),
        ] {
            for ty_chirho in ["[]", "Maybe", "Identity"] {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }

        // Functor/Applicative/Monad for ReadP, ReadPrec
        for (class_chirho, class_identity_chirho) in [
            (
                "Functor",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho),
            ),
            (
                "Applicative",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho),
            ),
            (
                "Monad",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho),
            ),
        ] {
            for ty_chirho in ["ReadP", "ReadPrec"] {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }

        // Eq/Ord/Show for Identity
        for (class_chirho, class_identity_chirho) in [
            (
                "Eq",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EqChirho),
            ),
            (
                "Ord",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::OrdChirho),
            ),
            (
                "Show",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ShowChirho),
            ),
            (
                "Read",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::ConChirho("Identity".to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }

        // Functor/Applicative/Monad for STM, Either, Proxy, First, Last,
        // Sum, Product, Dual, Down, Const, and other common types
        let common_monad_types_chirho: &[&str] = &[
            "STM", "Either", "Proxy", "First", "Last", "Sum", "Product", "Dual", "Down", "Const",
            "Min", "Max",
        ];
        for &ty_name_chirho in common_monad_types_chirho {
            for (class_chirho, class_identity_chirho) in [
                (
                    "Functor",
                    Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho),
                ),
                (
                    "Applicative",
                    Some(
                        haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
                    ),
                ),
                (
                    "Monad",
                    Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho),
                ),
            ] {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_name_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }
        // Foldable/Traversable for Either, Proxy, Identity, Const, Down
        for ty_chirho in ["Either", "Proxy", "Const", "Down"] {
            for (class_chirho, class_identity_chirho) in [
                (
                    "Foldable",
                    Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FoldableChirho),
                ),
                (
                    "Traversable",
                    Some(
                        haskelujah_ast_chirho::name_chirho::CompilerClassChirho::TraversableChirho,
                    ),
                ),
            ] {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }
        // MonadFail for STM
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadFailChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "MonadFail".to_string(),
                head_ty_chirho: TyChirho::ConChirho("STM".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        // Functor/Applicative/Monad for Q (Template Haskell)
        for (class_chirho, class_identity_chirho) in [
            (
                "Functor",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho),
            ),
            (
                "Applicative",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho),
            ),
            (
                "Monad",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho),
            ),
            (
                "MonadFail",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadFailChirho),
            ),
            (
                "MonadIO",
                Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadIoChirho),
            ),
        ] {
            self.add_seed_instance_reference_chirho(
                class_identity_chirho,
                crate::class_chirho::InstDeclPartsChirho {
                    kind_tys_chirho: Vec::new(),
                    class_name_chirho: class_chirho.to_string(),
                    head_ty_chirho: TyChirho::ConChirho("Q".to_string()),
                    extra_head_tys_chirho: vec![],
                    context_chirho: vec![],
                },
            );
        }
        // Semigroup/Monoid for common wrapper types
        for ty_chirho in [
            "First", "Last", "Sum", "Product", "Dual", "Endo", "Min", "Max", "All", "Any",
        ] {
            for (class_chirho, class_identity_chirho) in [
                (
                    "Semigroup",
                    Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::SemigroupChirho),
                ),
                (
                    "Monoid",
                    Some(haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonoidChirho),
                ),
            ] {
                self.add_seed_instance_reference_chirho(
                    class_identity_chirho,
                    crate::class_chirho::InstDeclPartsChirho {
                        kind_tys_chirho: Vec::new(),
                        class_name_chirho: class_chirho.to_string(),
                        head_ty_chirho: TyChirho::ConChirho(ty_chirho.to_string()),
                        extra_head_tys_chirho: vec![],
                        context_chirho: vec![],
                    },
                );
            }
        }

        // ── MonadTrans ──
        // class MonadTrans t where
        //   lift :: Monad m => m a -> t m a
        //
        // We represent the type as a simplified opaque:
        //   lift :: forall t m a. Monad m => m a -> t m a
        // Since our type system uses TyVarChirho, we encode this by giving
        // `lift` a type scheme with three type variables and a Monad pred.
        let mt_t_chirho = TyVarChirho(9060); // the transformer type constructor
        let mt_m_chirho = TyVarChirho(9061); // the inner monad
        let mt_a_chirho = TyVarChirho(9062); // the value type
        // m a
        let mt_ma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::VarChirho(mt_m_chirho)),
            Box::new(TyChirho::VarChirho(mt_a_chirho)),
        );
        // t m a
        let mt_tma_chirho = TyChirho::AppChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(mt_t_chirho)),
                Box::new(TyChirho::VarChirho(mt_m_chirho)),
            )),
            Box::new(TyChirho::VarChirho(mt_a_chirho)),
        );
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadTransChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "MonadTrans".to_string(),
                supers_chirho: vec![],
                var_chirho: mt_t_chirho,
                methods_chirho: HashMap::from([(
                    "lift".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![mt_t_chirho, mt_m_chirho, mt_a_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(mt_ma_chirho.clone()),
                            Box::new(mt_tma_chirho.clone()),
                            MultChirho::ManyChirho,
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // IsString (for OverloadedStrings extension)
        let is_string_var_chirho = TyVarChirho(9020);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IsStringChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "IsString".to_string(),
                supers_chirho: vec![],
                var_chirho: is_string_var_chirho,
                methods_chirho: HashMap::from([(
                    "fromString".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![is_string_var_chirho],
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::string_chirho()),
                            Box::new(TyChirho::VarChirho(is_string_var_chirho)),
                            MultChirho::ManyChirho,
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance IsString String  (identity)
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IsStringChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "IsString".to_string(),
                head_ty_chirho: TyChirho::string_chirho(),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IsStringChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "IsString".to_string(),
                head_ty_chirho: TyChirho::ConChirho("Builder".to_string()),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // IsList (for OverloadedLists extension)
        // Simplified: fromList :: [a] -> l, toList :: l -> [a]
        let is_list_var_chirho = TyVarChirho(9035);
        let is_list_item_var_chirho = TyVarChirho(9036);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IsListChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "IsList".to_string(),
                supers_chirho: vec![],
                var_chirho: is_list_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "fromList".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![is_list_var_chirho, is_list_item_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                                    is_list_item_var_chirho,
                                )))),
                                Box::new(TyChirho::VarChirho(is_list_var_chirho)),
                                MultChirho::ManyChirho,
                            ),
                        },
                    ),
                    (
                        "toList".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![is_list_var_chirho, is_list_item_var_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::VarChirho(is_list_var_chirho)),
                                Box::new(TyChirho::ListChirho(Box::new(TyChirho::VarChirho(
                                    is_list_item_var_chirho,
                                )))),
                                MultChirho::ManyChirho,
                            ),
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // instance IsList [a] (identity — fromList = id, toList = id)
        let a_var_chirho = TyVarChirho(9037);
        self.add_seed_instance_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IsListChirho,
            crate::class_chirho::InstDeclPartsChirho {
                kind_tys_chirho: Vec::new(),
                class_name_chirho: "IsList".to_string(),
                head_ty_chirho: TyChirho::ListChirho(Box::new(TyChirho::VarChirho(a_var_chirho))),
                extra_head_tys_chirho: vec![],
                context_chirho: vec![],
            },
        );

        // Generic (for DeriveGeneric extension)
        // class Generic a where
        //   from :: a -> Rep a
        //   to   :: Rep a -> a
        // Simplified: we use a single type variable and polymorphic methods.
        // The actual Rep type is encoded as Either/tuples at the value level
        // by the deriving mechanism.
        let generic_var_chirho = TyVarChirho(9090);
        let generic_rep_chirho = TyVarChirho(9091);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::GenericChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Generic".to_string(),
                supers_chirho: vec![],
                var_chirho: generic_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "from".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![generic_var_chirho, generic_rep_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::VarChirho(generic_var_chirho)),
                                Box::new(TyChirho::VarChirho(generic_rep_chirho)),
                                MultChirho::ManyChirho,
                            ),
                        },
                    ),
                    (
                        "to".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![generic_var_chirho, generic_rep_chirho],
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::VarChirho(generic_rep_chirho)),
                                Box::new(TyChirho::VarChirho(generic_var_chirho)),
                                MultChirho::ManyChirho,
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
