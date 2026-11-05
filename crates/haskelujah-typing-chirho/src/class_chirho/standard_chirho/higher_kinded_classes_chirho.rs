// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Contiguous standard-seed segment; order is owned by the parent dispatcher.

use std::collections::HashMap;

use crate::ty_chirho::{MultChirho, SchemeChirho, TyChirho, TyVarChirho};

use super::super::ClassEnvChirho;

impl ClassEnvChirho {
    pub(super) fn seed_higher_kinded_classes_chirho(&mut self) {
        // Functor (f :: * -> *)
        // fmap :: (a -> b) -> f a -> f b
        // (<$) :: a -> f b -> f a   (has default: fmap . const)
        let f_var_chirho = TyVarChirho(9004);
        let functor_a_chirho = TyVarChirho(9040);
        let functor_b_chirho = TyVarChirho(9041);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FunctorChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Functor".to_string(),
                supers_chirho: vec![],
                var_chirho: f_var_chirho,
                methods_chirho: HashMap::from([
                    // fmap :: (a -> b) -> f a -> f b
                    (
                        "fmap".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![f_var_chirho, functor_a_chirho, functor_b_chirho]
                                .into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::VarChirho(functor_a_chirho)),
                                    Box::new(TyChirho::VarChirho(functor_b_chirho)),
                                    MultChirho::ManyChirho,
                                )),
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(f_var_chirho)),
                                        Box::new(TyChirho::VarChirho(functor_a_chirho)),
                                    )),
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(f_var_chirho)),
                                        Box::new(TyChirho::VarChirho(functor_b_chirho)),
                                    )),
                                    MultChirho::ManyChirho,
                                )),
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

        // Applicative (superclass: Functor)
        // pure :: a -> f a
        // (<*>) :: f (a -> b) -> f a -> f b
        let app_var_chirho = TyVarChirho(9042);
        let app_a_chirho = TyVarChirho(9043);
        let app_b_chirho = TyVarChirho(9044);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ApplicativeChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Applicative".to_string(),
                supers_chirho: vec!["Functor".to_string()],
                var_chirho: app_var_chirho,
                methods_chirho: HashMap::from([
                    // pure :: a -> f a
                    (
                        "pure".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![app_var_chirho, app_a_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::VarChirho(app_a_chirho)),
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(app_var_chirho)),
                                    Box::new(TyChirho::VarChirho(app_a_chirho)),
                                )),
                                MultChirho::ManyChirho,
                            ),
                        },
                    ),
                    // (<*>) :: f (a -> b) -> f a -> f b
                    (
                        "<*>".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![app_var_chirho, app_a_chirho, app_b_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(app_var_chirho)),
                                    Box::new(TyChirho::FunChirho(
                                        Box::new(TyChirho::VarChirho(app_a_chirho)),
                                        Box::new(TyChirho::VarChirho(app_b_chirho)),
                                        MultChirho::ManyChirho,
                                    )),
                                )),
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(app_var_chirho)),
                                        Box::new(TyChirho::VarChirho(app_a_chirho)),
                                    )),
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(app_var_chirho)),
                                        Box::new(TyChirho::VarChirho(app_b_chirho)),
                                    )),
                                    MultChirho::ManyChirho,
                                )),
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

        // Monad (superclass: Applicative)
        // (>>=) :: m a -> (a -> m b) -> m b
        // return :: a -> m a  (default: pure)
        let m_var_chirho = TyVarChirho(9045);
        let monad_a_chirho = TyVarChirho(9046);
        let monad_b_chirho = TyVarChirho(9047);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::MonadChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Monad".to_string(),
                supers_chirho: vec!["Applicative".to_string()],
                var_chirho: m_var_chirho,
                methods_chirho: HashMap::from([
                    // (>>=) :: m a -> (a -> m b) -> m b
                    (
                        ">>=".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![m_var_chirho, monad_a_chirho, monad_b_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(m_var_chirho)),
                                    Box::new(TyChirho::VarChirho(monad_a_chirho)),
                                )),
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::FunChirho(
                                        Box::new(TyChirho::VarChirho(monad_a_chirho)),
                                        Box::new(TyChirho::AppChirho(
                                            Box::new(TyChirho::VarChirho(m_var_chirho)),
                                            Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                        )),
                                        MultChirho::ManyChirho,
                                    )),
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(m_var_chirho)),
                                        Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                    )),
                                    MultChirho::ManyChirho,
                                )),
                                MultChirho::ManyChirho,
                            ),
                        },
                    ),
                    // (>>) :: m a -> m b -> m b
                    (
                        ">>".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![m_var_chirho, monad_a_chirho, monad_b_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(m_var_chirho)),
                                    Box::new(TyChirho::VarChirho(monad_a_chirho)),
                                )),
                                Box::new(TyChirho::FunChirho(
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(m_var_chirho)),
                                        Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                    )),
                                    Box::new(TyChirho::AppChirho(
                                        Box::new(TyChirho::VarChirho(m_var_chirho)),
                                        Box::new(TyChirho::VarChirho(monad_b_chirho)),
                                    )),
                                    MultChirho::ManyChirho,
                                )),
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

        // Foldable (t :: * -> *)
        // foldMap :: Monoid m => (a -> m) -> t a -> m
        let foldable_t_chirho = TyVarChirho(9080);
        let foldable_a_chirho = TyVarChirho(9081);
        let foldable_m_chirho = TyVarChirho(9082);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::FoldableChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Foldable".to_string(),
                supers_chirho: vec![],
                var_chirho: foldable_t_chirho,
                methods_chirho: HashMap::from([(
                    "foldMap".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![foldable_t_chirho, foldable_a_chirho, foldable_m_chirho]
                            .into(),
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::FunChirho(
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::VarChirho(foldable_a_chirho)),
                                Box::new(TyChirho::VarChirho(foldable_m_chirho)),
                                MultChirho::ManyChirho,
                            )),
                            Box::new(TyChirho::FunChirho(
                                Box::new(TyChirho::AppChirho(
                                    Box::new(TyChirho::VarChirho(foldable_t_chirho)),
                                    Box::new(TyChirho::VarChirho(foldable_a_chirho)),
                                )),
                                Box::new(TyChirho::VarChirho(foldable_m_chirho)),
                                MultChirho::ManyChirho,
                            )),
                            MultChirho::ManyChirho,
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Traversable (superclass: Functor, Foldable)
        // traverse :: Applicative f => (a -> f b) -> t a -> f (t b)
        // sequenceA :: Applicative f => t (f a) -> f (t a)
        // mapM :: Monad m => (a -> m b) -> t a -> m (t b)
        // sequence :: Monad m => t (m a) -> m (t a)
        let trav_t_chirho = TyVarChirho(9083);
        let trav_a_chirho = TyVarChirho(9084);
        let trav_b_chirho = TyVarChirho(9085);
        let trav_f_chirho = TyVarChirho(9086);
        let traverse_ty_chirho = TyChirho::FunChirho(
            Box::new(TyChirho::FunChirho(
                Box::new(TyChirho::VarChirho(trav_a_chirho)),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(trav_f_chirho)),
                    Box::new(TyChirho::VarChirho(trav_b_chirho)),
                )),
                MultChirho::ManyChirho,
            )),
            Box::new(TyChirho::FunChirho(
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(trav_t_chirho)),
                    Box::new(TyChirho::VarChirho(trav_a_chirho)),
                )),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(trav_f_chirho)),
                    Box::new(TyChirho::AppChirho(
                        Box::new(TyChirho::VarChirho(trav_t_chirho)),
                        Box::new(TyChirho::VarChirho(trav_b_chirho)),
                    )),
                )),
                MultChirho::ManyChirho,
            )),
            MultChirho::ManyChirho,
        );
        let sequence_a_ty_chirho = TyChirho::FunChirho(
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(trav_t_chirho)),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(trav_f_chirho)),
                    Box::new(TyChirho::VarChirho(trav_a_chirho)),
                )),
            )),
            Box::new(TyChirho::AppChirho(
                Box::new(TyChirho::VarChirho(trav_f_chirho)),
                Box::new(TyChirho::AppChirho(
                    Box::new(TyChirho::VarChirho(trav_t_chirho)),
                    Box::new(TyChirho::VarChirho(trav_a_chirho)),
                )),
            )),
            MultChirho::ManyChirho,
        );
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::TraversableChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Traversable".to_string(),
                supers_chirho: vec!["Functor".to_string(), "Foldable".to_string()],
                var_chirho: trav_t_chirho,
                methods_chirho: HashMap::from([
                    (
                        "traverse".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![
                                trav_t_chirho,
                                trav_a_chirho,
                                trav_b_chirho,
                                trav_f_chirho,
                            ]
                            .into(),
                            preds_chirho: vec![],
                            ty_chirho: traverse_ty_chirho.clone(),
                        },
                    ),
                    (
                        "sequenceA".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![trav_t_chirho, trav_a_chirho, trav_f_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: sequence_a_ty_chirho.clone(),
                        },
                    ),
                    (
                        "mapM".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![
                                trav_t_chirho,
                                trav_a_chirho,
                                trav_b_chirho,
                                trav_f_chirho,
                            ]
                            .into(),
                            preds_chirho: vec![],
                            ty_chirho: traverse_ty_chirho,
                        },
                    ),
                    (
                        "sequence".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![trav_t_chirho, trav_a_chirho, trav_f_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: sequence_a_ty_chirho,
                        },
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Read (no superclasses in Haskell 2010)
        // Simplified: use `read` as the single method (String -> a)
        // instead of the full `readsPrec` / `readList` interface.
        let read_var_chirho = TyVarChirho(9011);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::ReadChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Read".to_string(),
                supers_chirho: vec![],
                var_chirho: read_var_chirho,
                methods_chirho: HashMap::from([(
                    "read".to_string(),
                    SchemeChirho {
                        vars_chirho: vec![read_var_chirho].into(),
                        preds_chirho: vec![],
                        ty_chirho: TyChirho::fun_chirho(
                            TyChirho::string_chirho(),
                            TyChirho::VarChirho(read_var_chirho),
                        ),
                    },
                )]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Test.Tasty.Options.IsOption
        let is_option_var_chirho = TyVarChirho(9190);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::IsOptionChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "IsOption".to_string(),
                supers_chirho: vec![],
                var_chirho: is_option_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "defaultValue".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![is_option_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::VarChirho(is_option_var_chirho),
                        },
                    ),
                    (
                        "parseValue".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![is_option_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::string_chirho(),
                                TyChirho::AppChirho(
                                    Box::new(TyChirho::ConChirho("Maybe".to_string())),
                                    Box::new(TyChirho::VarChirho(is_option_var_chirho)),
                                ),
                            ),
                        },
                    ),
                    (
                        "optionName".to_string(),
                        SchemeChirho::mono_chirho(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Maybe".to_string())),
                            Box::new(TyChirho::string_chirho()),
                        )),
                    ),
                    (
                        "optionHelp".to_string(),
                        SchemeChirho::mono_chirho(TyChirho::AppChirho(
                            Box::new(TyChirho::ConChirho("Maybe".to_string())),
                            Box::new(TyChirho::string_chirho()),
                        )),
                    ),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Enum: GHC's eight methods. The four enumerations are methods too, so a
        // polymorphic call takes its instance's own and a written instance
        // enumeration is checked against the class (row 671).
        // workflow: language-features-chirho/dictionary-evidence-chirho
        let enum_var_chirho = TyVarChirho(9006);
        let enumeration_chirho = |name_chirho: &str, arity_chirho: usize| {
            (
                name_chirho.to_string(),
                SchemeChirho {
                    vars_chirho: vec![enum_var_chirho].into(),
                    preds_chirho: vec![],
                    ty_chirho: TyChirho::fun_n_chirho(
                        vec![TyChirho::VarChirho(enum_var_chirho); arity_chirho],
                        TyChirho::ListChirho(Box::new(TyChirho::VarChirho(enum_var_chirho))),
                    ),
                },
            )
        };
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::EnumChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Enum".to_string(),
                supers_chirho: vec![],
                var_chirho: enum_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "toEnum".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![enum_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::int_chirho(),
                                TyChirho::VarChirho(enum_var_chirho),
                            ),
                        },
                    ),
                    (
                        "fromEnum".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![enum_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(enum_var_chirho),
                                TyChirho::int_chirho(),
                            ),
                        },
                    ),
                    (
                        "succ".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![enum_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(enum_var_chirho),
                                TyChirho::VarChirho(enum_var_chirho),
                            ),
                        },
                    ),
                    (
                        "pred".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![enum_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::fun_chirho(
                                TyChirho::VarChirho(enum_var_chirho),
                                TyChirho::VarChirho(enum_var_chirho),
                            ),
                        },
                    ),
                    enumeration_chirho("enumFrom", 1),
                    enumeration_chirho("enumFromThen", 2),
                    enumeration_chirho("enumFromTo", 2),
                    enumeration_chirho("enumFromThenTo", 3),
                ]),
                extra_vars_chirho: vec![],
                fundeps_chirho: vec![],
                defaults_chirho: HashMap::new(),
            },
        );

        // Bounded
        let bounded_var_chirho = TyVarChirho(9007);
        self.add_seed_class_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::BoundedChirho,
            crate::class_chirho::ClassDeclPartsChirho {
                kind_vars_chirho: Vec::new(),
                name_chirho: "Bounded".to_string(),
                supers_chirho: vec![],
                var_chirho: bounded_var_chirho,
                methods_chirho: HashMap::from([
                    (
                        "minBound".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bounded_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::VarChirho(bounded_var_chirho),
                        },
                    ),
                    (
                        "maxBound".to_string(),
                        SchemeChirho {
                            vars_chirho: vec![bounded_var_chirho].into(),
                            preds_chirho: vec![],
                            ty_chirho: TyChirho::VarChirho(bounded_var_chirho),
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
