// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `Control.Monad`, `Applicative`, `Arrow`, `Category` and `Debug`. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for `Control.Monad`, `Applicative`, `Arrow`, `Category` and `Debug`.
pub(super) fn push_control_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Control.Monad
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "when",
            "unless",
            "guard",
            "void",
            "join",
            "forM",
            "forM_",
            "mapM",
            "mapM_",
            "sequence",
            "sequence_",
            "forever",
            "foldM",
            "foldM_",
            "filterM",
            "zipWithM",
            "zipWithM_",
            "mplus",
            "mzero",
            "msum",
            "ap",
            "liftM",
            "liftM2",
            "return",
            ">>=",
            ">>",
            ">=>",
            "<=<",
            "fail",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (name_chirho, methods_chirho) in [
            ("Monad", &["return", ">>=", ">>"][..]),
            ("MonadPlus", &["mzero", "mplus"][..]),
            ("MonadFail", &["fail"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, methods_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Fix
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("mfix");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadFix", &["mfix"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Fix".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pure", "<*>", "*>", "<*", "liftA", "liftA2", "liftA3", "empty", "<|>", "some", "many",
            "optional",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Applicative", "Alternative"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Control.Arrow
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "arr", "first", "second", "***", "&&&", "returnA", "<<<", ">>>",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Arrow", "ArrowChoice", "ArrowApply", "ArrowLoop"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Arrow".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.IO.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("liftIO");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadIO", &["liftIO"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.IO.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Category
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["id", ".", "<<<", ">>>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Category", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Category".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Signatures (transformers internal)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["CallCC", "Catch", "Listen", "Pass"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Signatures".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative (base)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pure", "<*>", "*>", "<*", "<**>", "liftA", "liftA2", "liftA3", "empty", "<|>", "some",
            "many", "optional", "asum", "guard", "when", "unless",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Alternative", &["empty", "<|>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ZipList", &["ZipList"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad (base)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "return",
            ">>=",
            ">>",
            "=<<",
            "join",
            "void",
            "when",
            "unless",
            "guard",
            "forever",
            "mapM",
            "mapM_",
            "forM",
            "forM_",
            "sequence",
            "sequence_",
            "foldM",
            "foldM_",
            "filterM",
            "replicateM",
            "replicateM_",
            "liftM",
            "liftM2",
            "liftM3",
            "liftM4",
            "liftM5",
            "ap",
            "zipWithM",
            "zipWithM_",
            "foldM",
            "foldM_",
            "filterM",
            "MonadPlus",
            "mzero",
            "mplus",
            "msum",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "Monad".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Monad".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["return".to_string(), ">>=".to_string(), ">>".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "MonadPlus".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "MonadPlus".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["mzero".to_string(), "mplus".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "MonadFail".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "MonadFail".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec!["fail".to_string()],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Zip
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mzip", "mzipWith", "munzip"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MonadZip", &["mzipWith"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Zip".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative (17 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Applicative", &["pure", "<*>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Alternative", &["empty", "<|>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, mut v_chirho) = mk_type_chirho("Const", &["Const"]);
        v_chirho.methods_chirho.push("getConst".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, mut v_chirho) = mk_type_chirho("WrappedMonad", &["WrapMonad"]);
        v_chirho.methods_chirho.push("unwrapMonad".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, mut v_chirho) = mk_type_chirho("WrappedArrow", &["WrapArrow"]);
        v_chirho.methods_chirho.push("unwrapArrow".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, mut v_chirho) = mk_type_chirho("ZipList", &["ZipList"]);
        v_chirho.methods_chirho.push("getZipList".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "pure",
            "liftA",
            "liftA2",
            "liftA3",
            "<*>",
            "*>",
            "<*",
            "empty",
            "<|>",
            "some",
            "many",
            "optional",
            "getConst",
            "Const",
            "ZipList",
            "getZipList",
            "WrapMonad",
            "unwrapMonad",
            "WrapArrow",
            "unwrapArrow",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST (9 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // Control.Arrow (6 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Arrow", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowChoice", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowApply", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowZero", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowPlus", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowLoop", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Kleisli", &["Kleisli"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "arr",
            "first",
            "second",
            "***",
            "&&&",
            ">>>",
            "<<<",
            "returnA",
            "left",
            "right",
            "|||",
            "+++",
            "app",
            "Kleisli",
            "runKleisli",
            "loop",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Arrow".to_string(),
            exports_chirho,
        });
    }

    // Debug.Trace
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "trace",
            "traceShow",
            "traceShowId",
            "traceIO",
            "traceM",
            "traceShowM",
            "traceStack",
            "traceEvent",
            "traceEventIO",
            "traceMarker",
            "traceMarkerIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Debug.Trace".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Fail
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("fail");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadFail", &["fail"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Fail".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"ST";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.IO.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"MonadIO";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"liftIO";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.IO.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST.Lazy.Safe / Control.Monad.ST.Lazy.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ST",
            "runST",
            "fixST",
            "stToIO",
            "lazyToStrictST",
            "strictToLazyST",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST.Lazy.Safe".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST.Lazy.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST.Safe (re-export of Control.Monad.ST)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["ST", "runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST.Safe".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST / Control.Monad.ST.Unsafe / Control.Monad.ST.Lazy
    for mod_name_chirho in &[
        "Control.Monad.ST",
        "Control.Monad.ST.Unsafe",
        "Control.Monad.ST.Lazy",
        "Control.Monad.ST.Strict",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ST",
            "runST",
            "fixST",
            "stToIO",
            "strictToLazyST",
            "lazyToStrictST",
            "unsafeSTToIO",
            "unsafeIOToST",
            "unsafeInterleaveST",
            "RealWorld",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Combinators (parser-combinators package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["many", "some", "optional"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Combinators".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Instances (deprecated, empty — imports kept for compat)
    {
        let exports_chirho = IfaceExportsChirho::default();
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Instances".to_string(),
            exports_chirho,
        });
    }
}
