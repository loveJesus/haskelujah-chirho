// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `transformers` and unlift/resource modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for `transformers` and unlift/resource modules.
pub(super) fn push_transformers_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Control.Monad.Trans.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("lift");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadTrans", &["lift"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentityT", "mapIdentityT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IdentityT", &["IdentityT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Identity".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.State / .Lazy / .Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "runStateT",
            "evalStateT",
            "execStateT",
            "runState",
            "evalState",
            "execState",
            "get",
            "put",
            "modify",
            "gets",
            "state",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StateT", "State"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Control.Monad.Trans.State",
            "Control.Monad.Trans.State.Lazy",
            "Control.Monad.Trans.State.Strict",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: package_exposure_chirho(&["transformers"]),
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Reader
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runReaderT", "mapReaderT", "withReaderT", "reader"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ReaderT", &["ReaderT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Reader".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Writer / .Lazy / .Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "runWriterT",
            "execWriterT",
            "mapWriterT",
            "tell",
            "listen",
            "pass",
            "writer",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("WriterT", &["WriterT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &[
            "Control.Monad.Trans.Writer",
            "Control.Monad.Trans.Writer.Lazy",
            "Control.Monad.Trans.Writer.Strict",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: package_exposure_chirho(&["transformers"]),
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Except
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "runExceptT",
            "throwE",
            "catchE",
            "mapExceptT",
            "withExceptT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExceptT", &["ExceptT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Except".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Maybe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runMaybeT", "mapMaybeT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MaybeT", &["MaybeT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Maybe".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.* (transformers package — needed by mtl, everything)
    {
        // Common transformer types and operations
        let trans_modules_chirho = [
            (
                "Control.Monad.Trans.Class",
                vec!["lift"],
                vec![("MonadTrans", &[][..])],
            ),
            (
                "Control.Monad.Trans.Identity",
                vec!["runIdentityT", "liftCatch"],
                vec![("IdentityT", &["IdentityT"][..])],
            ),
            (
                "Control.Monad.Trans.Reader",
                vec![
                    "runReaderT",
                    "ask",
                    "local",
                    "asks",
                    "reader",
                    "withReaderT",
                    "mapReaderT",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                ],
                vec![("ReaderT", &["ReaderT"][..])],
            ),
            (
                "Control.Monad.Trans.State",
                vec![
                    "runStateT",
                    "evalStateT",
                    "execStateT",
                    "get",
                    "put",
                    "modify",
                    "gets",
                    "state",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                    "liftListen",
                    "liftPass",
                ],
                vec![("StateT", &["StateT"][..])],
            ),
            (
                "Control.Monad.Trans.State.Strict",
                vec![
                    "runStateT",
                    "evalStateT",
                    "execStateT",
                    "get",
                    "put",
                    "modify",
                    "gets",
                    "state",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                    "liftListen",
                    "liftPass",
                ],
                vec![("StateT", &["StateT"][..])],
            ),
            (
                "Control.Monad.Trans.State.Lazy",
                vec![
                    "runStateT",
                    "evalStateT",
                    "execStateT",
                    "get",
                    "put",
                    "modify",
                    "gets",
                    "state",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                    "liftListen",
                    "liftPass",
                ],
                vec![("StateT", &["StateT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                    "liftCatch",
                    "liftCallCC",
                    "liftListen",
                    "liftPass",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer.Strict",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                    "liftCatch",
                    "liftCallCC",
                    "liftListen",
                    "liftPass",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer.Lazy",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                    "liftCatch",
                    "liftCallCC",
                    "liftListen",
                    "liftPass",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Writer.CPS",
                vec![
                    "runWriterT",
                    "execWriterT",
                    "tell",
                    "listen",
                    "pass",
                    "writer",
                    "writerT",
                    "liftCatch",
                ],
                vec![("WriterT", &["WriterT"][..])],
            ),
            (
                "Control.Monad.Trans.Except",
                vec![
                    "runExceptT",
                    "throwE",
                    "catchE",
                    "mapExceptT",
                    "withExceptT",
                ],
                vec![("ExceptT", &["ExceptT"][..])],
            ),
            (
                "Control.Monad.Trans.Maybe",
                vec!["runMaybeT", "mapMaybeT", "liftCatch"],
                vec![("MaybeT", &["MaybeT"][..])],
            ),
            (
                "Control.Monad.Trans.Cont",
                vec![
                    "runContT",
                    "evalContT",
                    "cont",
                    "callCC",
                    "resetT",
                    "shiftT",
                    "liftCallCC",
                    "liftCallCC'",
                    "liftLocal",
                ],
                vec![("ContT", &["ContT"][..])],
            ),
            (
                "Control.Monad.Trans.RWS",
                vec![
                    "runRWST",
                    "evalRWST",
                    "execRWST",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                ],
                vec![("RWST", &["RWST"][..])],
            ),
            (
                "Control.Monad.Trans.RWS.Strict",
                vec![
                    "runRWST",
                    "evalRWST",
                    "execRWST",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                ],
                vec![("RWST", &["RWST"][..])],
            ),
            (
                "Control.Monad.Trans.RWS.Lazy",
                vec![
                    "runRWST",
                    "evalRWST",
                    "execRWST",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                ],
                vec![("RWST", &["RWST"][..])],
            ),
            (
                "Control.Monad.Trans.RWS.CPS",
                vec![
                    "runRWST",
                    "evalRWST",
                    "execRWST",
                    "liftCatch",
                    "liftCallCC",
                    "liftCallCC'",
                ],
                vec![("RWST", &["RWST"][..])],
            ),
        ];
        for (mod_name_chirho, vals_chirho, types_chirho) in &trans_modules_chirho {
            let mut exports_chirho = IfaceExportsChirho::default();
            for val_chirho in vals_chirho {
                let (k_chirho, v_chirho) = mk_val_chirho(val_chirho);
                exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            }
            for (ty_name_chirho, cons_chirho) in types_chirho {
                let (k_chirho, v_chirho) = mk_type_chirho(ty_name_chirho, cons_chirho);
                exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            }
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: package_exposure_chirho(&["transformers"]),
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho,
            });
        }
    }

    // Control.Monad.Trans.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("MonadTrans", &["lift"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("lift");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans — umbrella re-export module
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["lift", "liftIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (name_chirho, methods_chirho) in
            [("MonadTrans", &["lift"][..]), ("MonadIO", &["liftIO"][..])]
        {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, methods_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Trans".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.State / Control.Monad.Trans.State.Lazy
    for mod_name_chirho in &[
        "Control.Monad.Trans.State",
        "Control.Monad.Trans.State.Lazy",
        "Control.Monad.Trans.State.Strict",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "get",
            "put",
            "modify",
            "modify'",
            "gets",
            "evalState",
            "execState",
            "runState",
            "evalStateT",
            "execStateT",
            "runStateT",
            "state",
            "withState",
            "mapState",
            "mapStateT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StateT", "State"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["StateT"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Writer / Control.Monad.Trans.Writer.Lazy
    for mod_name_chirho in &[
        "Control.Monad.Trans.Writer",
        "Control.Monad.Trans.Writer.Lazy",
        "Control.Monad.Trans.Writer.Strict",
        "Control.Monad.Trans.Writer.CPS",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "tell",
            "listen",
            "pass",
            "censor",
            "runWriter",
            "execWriter",
            "runWriterT",
            "execWriterT",
            "writer",
            "writerT",
            "mapWriter",
            "mapWriterT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["WriterT", "Writer"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["WriterT"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative.Backwards / Control.Applicative.Lift
    for mod_name_chirho in &["Control.Applicative.Backwards", "Control.Applicative.Lift"] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Backwards",
            "forwards",
            "Lift",
            "unLift",
            "runLift",
            "mapLift",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, mut v_chirho) = mk_type_chirho("Backwards", &["Backwards"]);
        v_chirho.methods_chirho.push("forwards".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Lift", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Accum / Control.Monad.Trans.Select
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "AccumT",
            "runAccumT",
            "execAccumT",
            "evalAccumT",
            "mapAccumT",
            "Accum",
            "runAccum",
            "execAccum",
            "evalAccum",
            "accum",
            "look",
            "looks",
            "add",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("AccumT", &["AccumT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Accum".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "SelectT",
            "runSelectT",
            "mapSelectT",
            "Select",
            "runSelect",
            "select",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        exports_chirho.types_chirho.insert(
            "SelectT".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "SelectT".to_string(),
                constructors_chirho: vec!["SelectT".to_string()],
                methods_chirho: vec!["runSelectT".to_string()],
                is_class_chirho: false,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["transformers"]),
            name_chirho: "Control.Monad.Trans.Select".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Error / Control.Monad.Trans.List (deprecated but still imported)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ErrorT",
            "runErrorT",
            "mapErrorT",
            "Error",
            "noMsg",
            "strMsg",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ErrorT", &["ErrorT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Error", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.Monad.Trans.Error".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["ListT", "runListT", "mapListT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ListT", &["ListT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.Monad.Trans.List".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Resource (resourcet)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ResourceT",
            "runResourceT",
            "liftResourceT",
            "transResourceT",
            "MonadResource",
            "liftResourceT",
            "allocate",
            "register",
            "release",
            "resourceForkIO",
            "ReleaseKey",
            "MonadUnliftIO",
            "withRunInIO",
            "UnliftIO",
            "askUnliftIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "ResourceT",
            "MonadResource",
            "ReleaseKey",
            "MonadUnliftIO",
            "UnliftIO",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Control.Monad.Trans.Resource",
            "Control.Monad.Trans.Resource.Internal",
            "UnliftIO",
            "Control.Monad.IO.Unlift",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Instances (transformers-compat — re-exports)
    {
        let exports_chirho = IfaceExportsChirho::default();
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.Monad.Trans.Instances".to_string(),
            exports_chirho,
        });
    }
}
