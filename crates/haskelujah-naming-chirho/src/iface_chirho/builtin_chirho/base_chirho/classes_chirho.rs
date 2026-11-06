// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: the `Functor`, `Foldable`, `Traversable`, `Monoid` and `Semigroup` modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for the `Functor`, `Foldable`, `Traversable`, `Monoid` and `Semigroup` modules.
pub(super) fn push_classes_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        exports_chirho.types_chirho.insert(
            "Identity".to_string(),
            IfaceTypeChirho {
                name_chirho: "Identity".to_string(),
                constructors_chirho: vec!["Identity".to_string()],
                methods_chirho: vec!["runIdentity".to_string()],
                is_class_chirho: false,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        let (k_chirho, v_chirho) = mk_val_chirho("Identity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("runIdentity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Sum.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Sum2", "Sum3", "Sum4", "Sum5"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sum.Experimental".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mempty",
            "mappend",
            "mconcat",
            "getSum",
            "getProduct",
            "getFirst",
            "getLast",
            "getAny",
            "getAll",
            "getMin",
            "getMax",
            "getDual",
            "getEndo",
            "appEndo",
            "getAlt",
            "getAp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Monoid", "Endo"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Newtypes with record accessors — list constructor + accessor
        for (type_chirho, ctors_chirho) in &[
            ("Sum", &["Sum", "getSum"][..]),
            ("Product", &["Product", "getProduct"][..]),
            ("First", &["First", "getFirst"][..]),
            ("Last", &["Last", "getLast"][..]),
            ("Any", &["Any", "getAny"][..]),
            ("All", &["All", "getAll"][..]),
            ("Min", &["Min", "getMin"][..]),
            ("Max", &["Max", "getMax"][..]),
            ("Dual", &["Dual", "getDual"][..]),
            ("Ap", &["Ap", "getAp"][..]),
            ("Alt", &["Alt", "getAlt"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "(<>)",
            "sconcat",
            "stimes",
            "getMin",
            "getMax",
            "getFirst",
            "getLast",
            "getWrappedMonoid",
            "unwrapMonoid",
            "getOption",
            "option",
            "diff",
            "cycle1",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Semigroup";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["(<>)", "sconcat", "stimes"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for (type_chirho, ctors_chirho) in &[
            ("Min", &["Min", "getMin"][..]),
            ("Max", &["Max", "getMax"][..]),
            ("First", &["First", "getFirst"][..]),
            ("Last", &["Last", "getLast"][..]),
            (
                "WrappedMonoid",
                &["WrapMonoid", "unwrapMonoid", "getWrappedMonoid"][..],
            ),
            ("Option", &["Option", "getOption"][..]),
            ("Arg", &["Arg"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Semigroup".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["fmap", "<$>", "<$", "$>", "void", "<&>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &["fmap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fold",
            "foldMap",
            "foldl",
            "foldr",
            "foldl'",
            "foldr'",
            "toList",
            "null",
            "length",
            "elem",
            "maximum",
            "minimum",
            "sum",
            "product",
            "any",
            "all",
            "and",
            "or",
            "concat",
            "concatMap",
            "asum",
            "find",
            "mapM_",
            "forM_",
            "sequence_",
            "for_",
            "traverse_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable", &["foldMap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Traversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "traverse",
            "sequenceA",
            "mapM",
            "sequence",
            "for",
            "forM",
            "mapAccumL",
            "mapAccumR",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho(
            "Traversable",
            &["traverse", "sequenceA", "mapM", "sequence"],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Traversable".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Contravariant
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "contramap",
            "phantom",
            ">$<",
            ">$$<",
            "Predicate",
            "getPredicate",
            "Comparison",
            "getComparison",
            "Equivalence",
            "getEquivalence",
            "Op",
            "getOp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Contravariant", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Contravariant".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"runIdentity";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Identity".to_string(),
            IfaceTypeChirho {
                name_chirho: "Identity".to_string(),
                constructors_chirho: vec!["Identity".to_string()],
                methods_chirho: vec!["runIdentity".to_string()],
                is_class_chirho: false,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"getCompose";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, mut v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        v_chirho.methods_chirho.push("getCompose".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Product
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Pair"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Product".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Sum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["InL", "InR"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Sum".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"getConst";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Const");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getConst");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Classes
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "eq1",
            "compare1",
            "showsPrec1",
            "readsPrec1",
            "liftEq",
            "liftCompare",
            "liftShowsPrec",
            "liftReadsPrec",
            "eq2",
            "compare2",
            "showsPrec2",
            "readsPrec2",
            "liftEq2",
            "liftCompare2",
            "liftShowsPrec2",
            "liftReadsPrec2",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Eq1", "Ord1", "Show1", "Read1", "Eq2", "Ord2", "Show2", "Read2",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Classes".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, mut v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        v_chirho.methods_chirho.push("getCompose".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Compose");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getCompose");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Product
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Pair"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Pair");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Product".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Sum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["InL", "InR"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["InL", "InR"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Sum".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity (12 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Identity", "runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup (used by some test files)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Semigroup", &["<>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Min", &["Min"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Max", &["Max"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("First", &["First"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Last", &["Last"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Arg", &["Arg"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "<>", "sconcat", "stimes", "Min", "getMin", "Max", "getMax", "First", "getFirst",
            "Last", "getLast", "Arg",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Semigroup".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Monoid", &["mappend", "mempty", "mconcat"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Dual", &["Dual"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Endo", &["Endo"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["Sum"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Product"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("All", &["All"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Any", &["Any"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Ap", &["Ap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Alt", &["Alt"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "mempty",
            "mappend",
            "mconcat",
            "Dual",
            "getDual",
            "Endo",
            "appEndo",
            "Sum",
            "getSum",
            "Product",
            "getProduct",
            "All",
            "getAll",
            "Any",
            "getAny",
            "Ap",
            "getAp",
            "Alt",
            "getAlt",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor (common import)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &["fmap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["fmap", "<$>", "<$", "$>", "void", "<&>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Const", "getConst"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Compose", "getCompose"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable", &["foldMap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "fold",
            "foldMap",
            "foldMap'",
            "foldr",
            "foldl",
            "foldl'",
            "foldr'",
            "toList",
            "null",
            "length",
            "elem",
            "maximum",
            "minimum",
            "sum",
            "product",
            "and",
            "or",
            "any",
            "all",
            "concat",
            "concatMap",
            "find",
            "asum",
            "mapM_",
            "forM_",
            "sequenceA_",
            "sequence_",
            "traverse_",
            "for_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Traversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho(
            "Traversable",
            &["traverse", "sequenceA", "mapM", "sequence"],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "traverse",
            "sequenceA",
            "mapM",
            "sequence",
            "for",
            "forM",
            "mapAccumL",
            "mapAccumR",
            "fmapDefault",
            "foldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Traversable".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"runIdentity";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Identity".to_string(),
            IfaceTypeChirho {
                name_chirho: "Identity".to_string(),
                constructors_chirho: vec!["Identity".to_string()],
                methods_chirho: vec!["runIdentity".to_string()],
                is_class_chirho: false,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable (extend existing with additional exports)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fold",
            "foldMap",
            "foldMap'",
            "foldr",
            "foldr'",
            "foldl",
            "foldl'",
            "toList",
            "null",
            "length",
            "elem",
            "maximum",
            "minimum",
            "sum",
            "product",
            "any",
            "all",
            "and",
            "or",
            "find",
            "notElem",
            "maximumBy",
            "minimumBy",
            "concat",
            "concatMap",
            "asum",
            "traverse_",
            "for_",
            "mapM_",
            "forM_",
            "sequenceA_",
            "sequence_",
            "foldrM",
            "foldlM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Foldable";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"Compose";
            let (k_chirho, mut v_chirho) = mk_type_chirho(name_chirho, &["Compose"]);
            v_chirho.methods_chirho.push("getCompose".to_string());
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Compose", "getCompose"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"Const";
            let (k_chirho, mut v_chirho) = mk_type_chirho(name_chirho, &["Const"]);
            v_chirho.methods_chirho.push("getConst".to_string());
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Const", "getConst"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid (extend)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Monoid", "Dual", "Endo", "All", "Any", "Sum", "Product", "Min", "Max", "First",
            "Last", "Ap", "Alt",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "mempty",
            "mappend",
            "mconcat",
            "Dual",
            "getDual",
            "Endo",
            "appEndo",
            "All",
            "getAll",
            "Any",
            "getAny",
            "Sum",
            "getSum",
            "Product",
            "getProduct",
            "Min",
            "getMin",
            "Max",
            "getMax",
            "First",
            "getFirst",
            "Last",
            "getLast",
            "Ap",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Reverse (transformers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Reverse", "getReverse"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, mut v_chirho) = mk_type_chirho("Reverse", &["Reverse"]);
        v_chirho.methods_chirho.push("getReverse".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Reverse".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Constant (transformers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Constant", "getConstant"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, mut v_chirho) = mk_type_chirho("Constant", &["Constant"]);
        v_chirho.methods_chirho.push("getConstant".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Constant".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Adjunction (adjunctions package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Adjunction",
            "unit",
            "counit",
            "leftAdjunct",
            "rightAdjunct",
            "tabulateAdjunction",
            "indexAdjunction",
            "zapWithAdjunction",
            "splitL",
            "unsplitL",
            "extractL",
            "duplicateL",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Adjunction", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Adjunction".to_string(),
            exports_chirho,
        });
    }
}
