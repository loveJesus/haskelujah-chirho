// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: comonads, semigroupoids, bifunctors, indexed traversals, lenses and profunctors. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for comonads, semigroupoids, bifunctors, indexed traversals, lenses and profunctors.
pub(super) fn push_functors_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Bifunctor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bimap", "first", "second"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifunctor", &["bimap", "first", "second"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Bifunctor".to_string(),
            exports_chirho,
        });
    }

    // Control.Lens (lens package — massive, synthetic)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "view",
            "over",
            "set",
            "lens",
            "iso",
            "prism",
            "traverse",
            "to",
            "from",
            "review",
            "preview",
            "folded",
            "filtered",
            "mapped",
            "each",
            "both",
            "ix",
            "at",
            "contains",
            "_1",
            "_2",
            "_3",
            "_4",
            "_5",
            "_head",
            "_tail",
            "_init",
            "_last",
            "_Left",
            "_Right",
            "_Just",
            "_Nothing",
            "makeLenses",
            "makePrisms",
            "makeClassy",
            "makeFields",
            "(&)",
            "(%~)",
            "(.~)",
            "(^.)",
            "(^?)",
            "(^..)",
            "(+~)",
            "(-~)",
            "(*~)",
            "(//~)",
            "(&&~)",
            "(||~)",
            "use",
            "uses",
            "assign",
            "modifying",
            "zoom",
            "magnify",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for ty_chirho in &[
            ("Lens", &[][..]),
            ("Lens'", &[]),
            ("Traversal", &[]),
            ("Traversal'", &[]),
            ("Prism", &[]),
            ("Prism'", &[]),
            ("Iso", &[]),
            ("Iso'", &[]),
            ("Getter", &[]),
            ("Setter", &[]),
            ("Setter'", &[]),
            ("Fold", &[]),
            ("Review", &[]),
            ("Getting", &[]),
            ("ASetter", &[]),
            ("ASetter'", &[]),
            ("ALens", &[]),
            ("ALens'", &[]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(ty_chirho.0, ty_chirho.1);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let lens_modules_chirho = [
            "Control.Lens",
            "Control.Lens.Type",
            "Control.Lens.Lens",
            "Control.Lens.Getter",
            "Control.Lens.Setter",
            "Control.Lens.Fold",
            "Control.Lens.Traversal",
            "Control.Lens.Prism",
            "Control.Lens.Iso",
            "Control.Lens.Review",
            "Control.Lens.At",
            "Control.Lens.Each",
            "Control.Lens.Indexed",
            "Control.Lens.TH",
            "Control.Lens.Combinators",
            "Control.Lens.Operators",
            "Control.Lens.Tuple",
        ];
        for mod_name_chirho in &lens_modules_chirho {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Bifoldable1
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bifoldMap1", "bifold1"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifoldable1", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Bifoldable1".to_string(),
            exports_chirho,
        });
    }

    // Data.Bifoldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "bifold",
            "bifoldMap",
            "bifoldr",
            "bifoldl",
            "bifoldl'",
            "bifoldr'",
            "biList",
            "biany",
            "biall",
            "biconcat",
            "biconcatMap",
            "bitraverse_",
            "bifor_",
            "bimapM_",
            "bisequenceA_",
            "bisequence_",
            "binull",
            "bilength",
            "bielem",
            "bimaximum",
            "biminimum",
            "bisum",
            "biproduct",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifoldable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Bifoldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Bitraversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "bitraverse",
            "bisequenceA",
            "bimapM",
            "bifor",
            "bimapDefault",
            "bifoldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bitraversable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Bitraversable".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable1
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "foldMap1",
            "fold1",
            "toNonEmpty",
            "maximum1",
            "minimum1",
            "head1",
            "last1",
            "foldrMap1",
            "foldlMap1",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable1", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Foldable1".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup.Foldable / Data.Semigroup.Traversable compat surface
    for mod_name_chirho in &[
        "Data.Semigroup.Foldable",
        "Data.Semigroup.Traversable",
        "Data.Semigroup.Traversable.Class",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "foldMap1",
            "fold1",
            "toNonEmpty",
            "maximum1",
            "minimum1",
            "head1",
            "last1",
            "foldrMap1",
            "foldlMap1",
            "traverse1",
            "sequence1",
            "traverse1Maybe",
            "gtraverse1",
            "gsequence1",
            "foldMap1Default",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable1", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        exports_chirho.types_chirho.insert(
            "Traversable1".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Traversable1".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["traverse1".to_string(), "sequence1".to_string()],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: (*mod_name_chirho).to_string(),
            exports_chirho,
        });
    }

    // Data.Default (data-default)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("def");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Default", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Data.Default", "Data.Default.Class"] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Functor.Apply / Data.Functor.Bind / Data.Functor.Alt / Data.Functor.Plus (semigroupoids)
    for mod_name_chirho in &[
        "Data.Functor.Apply",
        "Data.Functor.Bind",
        "Data.Functor.Bind.Class",
        "Data.Functor.Alt",
        "Data.Functor.Plus",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Apply",
            "<.>",
            ".>",
            "<.",
            "liftF2",
            "liftF3",
            "Bind",
            ">>-",
            "join",
            "Alt",
            "<!>",
            "some",
            "many",
            "Plus",
            "zero",
            "MaybeApply",
            "WrappedApplicative",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Apply".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Apply".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "<.>".to_string(),
                    ".>".to_string(),
                    "<.".to_string(),
                    "liftF2".to_string(),
                ],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "Bind".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Bind".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![">>-".to_string(), "join".to_string()],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "Alt".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Alt".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["<!>".to_string(), "some".to_string(), "many".to_string()],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "Plus".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Plus".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["zero".to_string()],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        for name_chirho in &["MaybeApply", "WrappedApplicative"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Comonad / Control.Comonad.Trans.Traced / etc (comonad package)
    for mod_name_chirho in &[
        "Control.Comonad",
        "Control.Comonad.Env",
        "Control.Comonad.Env.Class",
        "Control.Comonad.Store",
        "Control.Comonad.Store.Class",
        "Control.Comonad.Traced",
        "Control.Comonad.Traced.Class",
        "Control.Comonad.Trans.Env",
        "Control.Comonad.Trans.Traced",
        "Control.Comonad.Trans.Store",
        "Control.Comonad.Trans.Class",
        "Control.Comonad.Hoist.Class",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Comonad",
            "extract",
            "duplicate",
            "extend",
            "=>>",
            "liftW",
            "wfix",
            "cfix",
            "kfix",
            "ComonadApply",
            "<@>",
            "<@@>",
            "liftW2",
            "liftW3",
            "EnvT",
            "runEnvT",
            "env",
            "ask",
            "asks",
            "local",
            "StoreT",
            "runStoreT",
            "store",
            "pos",
            "peek",
            "peeks",
            "seek",
            "seeks",
            "experiment",
            "TracedT",
            "runTracedT",
            "traced",
            "trace",
            "listen",
            "listens",
            "censor",
            "ComonadTrans",
            "lower",
            "ComonadHoist",
            "cohoist",
            "ComonadEnv",
            "ComonadStore",
            "ComonadTraced",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Comonad",
            "ComonadApply",
            "ComonadTrans",
            "ComonadHoist",
            "ComonadEnv",
            "ComonadStore",
            "ComonadTraced",
            "EnvT",
            "StoreT",
            "TracedT",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Data.Tagged (tagged package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Tagged",
            "unTagged",
            "retag",
            "untag",
            "tagSelf",
            "untagSelf",
            "asTaggedTypeOf",
            "witness",
            "proxy",
            "tagWith",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Tagged", &["Tagged"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Data.Tagged".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.WithIndex / Data.Foldable.WithIndex / Data.Traversable.WithIndex (indexed-traversable)
    for mod_name_chirho in &[
        "Data.Functor.WithIndex",
        "Data.Foldable.WithIndex",
        "Data.Traversable.WithIndex",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "FunctorWithIndex",
            "imap",
            "FoldableWithIndex",
            "ifoldMap",
            "ifoldr",
            "ifoldl",
            "ifoldr'",
            "ifoldl'",
            "itraverse_",
            "ifor_",
            "TraversableWithIndex",
            "itraverse",
            "ifor",
            "imapDefault",
            "ifoldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "FunctorWithIndex",
            "FoldableWithIndex",
            "TraversableWithIndex",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    for mod_name_chirho in &[
        "Data.Functor.WithIndex.Instances",
        "Data.Foldable.WithIndex.Instances",
        "Data.Traversable.WithIndex.Instances",
    ] {
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho: IfaceExportsChirho::default(),
        });
    }

    // Data.Bifunctor.Swap (bifunctors)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Swap", "unSwap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Swap", &["Swap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Data.Bifunctor.Swap".to_string(),
            exports_chirho,
        });
    }

    // Data.Profunctor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["dimap", "lmap", "rmap", "arr'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Profunctor", &["dimap", "lmap", "rmap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for sub_mod_chirho in &[
            "Data.Profunctor",
            "Data.Profunctor.Unsafe",
            "Data.Profunctor.Types",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: sub_mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Bifunctor.Assoc / Data.Bifunctor.Swap (assoc package)
    for mod_name_chirho in &["Data.Bifunctor.Assoc", "Data.Bifunctor.Swap"] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["assoc", "unassoc", "swap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Assoc", "Swap"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["assoc", "unassoc", "swap"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Data.Bifunctor.Functor (bifunctor-classes-compat)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["QualifiedBifunctor", "WrappedBifunctor"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Data.Bifunctor.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Bifunctor.Tannen / Biff / Clown / Joker / Flip / Product / Sum
    // (bifunctors package, needed by lens)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Tannen",
            "Biff",
            "Clown",
            "Joker",
            "Flip",
            "Product",
            "Sum",
            "runTannen",
            "runBiff",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Data.Bifunctor.Tannen",
            "Data.Bifunctor.Biff",
            "Data.Bifunctor.Clown",
            "Data.Bifunctor.Joker",
            "Data.Bifunctor.Flip",
            "Data.Bifunctor.Product",
            "Data.Bifunctor.Sum",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }
}
