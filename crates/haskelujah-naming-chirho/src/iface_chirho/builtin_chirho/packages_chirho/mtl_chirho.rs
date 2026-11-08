// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `mtl`'s monad classes. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `mtl`'s monad classes.
pub(super) fn push_mtl_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Control.Monad.Reader
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ask",
            "asks",
            "local",
            "reader",
            "runReader",
            "runReaderT",
            "mapReaderT",
            "withReaderT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Reader", "ReaderT", "MonadReader"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Reader".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.State / Control.Monad.State.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "get",
            "put",
            "modify",
            "gets",
            "state",
            "runState",
            "evalState",
            "execState",
            "runStateT",
            "evalStateT",
            "execStateT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["State", "StateT"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        {
            let (k_chirho, v_chirho) = mk_type_chirho("MonadState", &["get", "put", "state"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.State".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.State.Class".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.State.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.State.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Writer / Control.Monad.Writer.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "tell",
            "listen",
            "pass",
            "writer",
            "runWriter",
            "execWriter",
            "runWriterT",
            "execWriterT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Writer", "WriterT", "MonadWriter"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Writer".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Writer.Class".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Writer.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Writer.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Except
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throwError",
            "catchError",
            "runExcept",
            "runExceptT",
            "mapExcept",
            "mapExceptT",
            "withExcept",
            "withExceptT",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Except", "ExceptT", "MonadError"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Except".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"runIdentity";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Identity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: "Control.Monad.Identity".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Catch (exceptions package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throwM",
            "catch",
            "catchAll",
            "try",
            "tryJust",
            "handle",
            "handleAll",
            "handleJust",
            "onException",
            "bracket",
            "bracket_",
            "finally",
            "mask",
            "uninterruptibleMask",
            "mask_",
            "catches",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MonadThrow", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadCatch", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadMask", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SomeException", &["SomeException"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["exceptions"]),
            name_chirho: "Control.Monad.Catch".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Primitive
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "PrimMonad",
            "primitive",
            "PrimState",
            "PrimBase",
            "RealWorld",
            "primToPrim",
            "primToIO",
            "primToST",
            "stToPrim",
            "ioToPrim",
            "liftPrim",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["PrimMonad", "PrimBase"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.Monad.Primitive".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.RWS / Control.Monad.RWS.Strict / Control.Monad.RWS.Lazy / Control.Monad.RWS.Class
    for mod_name_chirho in &[
        "Control.Monad.RWS",
        "Control.Monad.RWS.Strict",
        "Control.Monad.RWS.Lazy",
        "Control.Monad.RWS.Class",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "RWS",
            "RWST",
            "runRWS",
            "runRWST",
            "evalRWS",
            "evalRWST",
            "execRWS",
            "execRWST",
            "mapRWS",
            "mapRWST",
            "withRWS",
            "withRWST",
            "MonadReader",
            "ask",
            "local",
            "reader",
            "asks",
            "MonadWriter",
            "tell",
            "listen",
            "pass",
            "writer",
            "censor",
            "MonadState",
            "get",
            "put",
            "state",
            "modify",
            "modify'",
            "gets",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["RWS", "RWST", "MonadReader", "MonadWriter", "MonadState"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["mtl"]),
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Reader / Control.Monad.Writer / Control.Monad.State / Control.Monad.Except / Control.Monad.Error (mtl wrappers)
    for (mod_name_chirho, exposure_chirho) in &[
        ("Control.Monad.Reader", package_exposure_chirho(&["mtl"])),
        (
            "Control.Monad.Reader.Class",
            package_exposure_chirho(&["mtl"]),
        ),
        ("Control.Monad.Writer", package_exposure_chirho(&["mtl"])),
        (
            "Control.Monad.Writer.Strict",
            package_exposure_chirho(&["mtl"]),
        ),
        (
            "Control.Monad.Writer.Lazy",
            package_exposure_chirho(&["mtl"]),
        ),
        (
            "Control.Monad.Writer.Class",
            package_exposure_chirho(&["mtl"]),
        ),
        ("Control.Monad.State", package_exposure_chirho(&["mtl"])),
        (
            "Control.Monad.State.Strict",
            package_exposure_chirho(&["mtl"]),
        ),
        (
            "Control.Monad.State.Lazy",
            package_exposure_chirho(&["mtl"]),
        ),
        (
            "Control.Monad.State.Class",
            package_exposure_chirho(&["mtl"]),
        ),
        ("Control.Monad.Except", package_exposure_chirho(&["mtl"])),
        ("Control.Monad.Error", None),
        (
            "Control.Monad.Error.Class",
            package_exposure_chirho(&["mtl"]),
        ),
        ("Control.Monad.Cont", package_exposure_chirho(&["mtl"])),
        (
            "Control.Monad.Cont.Class",
            package_exposure_chirho(&["mtl"]),
        ),
        (
            "Control.Monad.Writer.CPS",
            package_exposure_chirho(&["mtl"]),
        ),
        ("Control.Monad.RWS.CPS", package_exposure_chirho(&["mtl"])),
        ("Control.Monad.Identity", package_exposure_chirho(&["mtl"])),
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "MonadReader",
            "ask",
            "local",
            "reader",
            "asks",
            "ReaderT",
            "runReaderT",
            "mapReaderT",
            "withReaderT",
            "Reader",
            "runReader",
            "mapReader",
            "withReader",
            "MonadWriter",
            "tell",
            "listen",
            "pass",
            "writer",
            "writerT",
            "censor",
            "WriterT",
            "runWriterT",
            "execWriterT",
            "mapWriterT",
            "Writer",
            "runWriter",
            "execWriter",
            "mapWriter",
            "MonadState",
            "get",
            "put",
            "state",
            "modify",
            "modify'",
            "gets",
            "StateT",
            "runStateT",
            "evalStateT",
            "execStateT",
            "mapStateT",
            "withStateT",
            "State",
            "runState",
            "evalState",
            "execState",
            "mapState",
            "withState",
            "MonadError",
            "throwError",
            "catchError",
            "ExceptT",
            "runExceptT",
            "mapExceptT",
            "withExceptT",
            "Except",
            "runExcept",
            "mapExcept",
            "withExcept",
            "ErrorT",
            "runErrorT",
            "mapErrorT",
            "Error",
            "noMsg",
            "strMsg",
            "MonadCont",
            "callCC",
            "ContT",
            "runContT",
            "mapContT",
            "withContT",
            "Cont",
            "runCont",
            "mapCont",
            "withCont",
            "liftCallCC",
            "liftCallCC'",
            "Identity",
            "runIdentity",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "MonadReader",
            "ReaderT",
            "Reader",
            "MonadWriter",
            "WriterT",
            "Writer",
            "MonadState",
            "StateT",
            "State",
            "MonadError",
            "ExceptT",
            "Except",
            "ErrorT",
            "Error",
            "MonadCont",
            "ContT",
            "Cont",
            "Identity",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: exposure_chirho.clone(),
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
}
