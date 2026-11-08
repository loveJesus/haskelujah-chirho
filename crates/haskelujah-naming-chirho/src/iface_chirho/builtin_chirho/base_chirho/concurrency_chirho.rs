// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: concurrency, exceptions, parallelism and `deepseq`. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for concurrency, exceptions, parallelism and `deepseq`.
pub(super) fn push_concurrency_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Control.Concurrent.STM
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newTVar",
            "readTVar",
            "writeTVar",
            "modifyTVar",
            "modifyTVar'",
            "swapTVar",
            "newTVarIO",
            "readTVarIO",
            "newTMVar",
            "readTMVar",
            "putTMVar",
            "takeTMVar",
            "newTMVarIO",
            "newEmptyTMVar",
            "newEmptyTMVarIO",
            "atomically",
            "retry",
            "orElse",
            "check",
            "throwSTM",
            "catchSTM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STM", "TVar", "TMVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["stm"]),
            name_chirho: "Control.Concurrent.STM".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Also expose as GHC.Conc / Control.Concurrent.STM.TVar
        for (sub_mod_chirho, exposure_chirho) in &[
            (
                "Control.Concurrent.STM.TVar",
                package_exposure_chirho(&["stm"]),
            ),
            (
                "Control.Concurrent.STM.TMVar",
                package_exposure_chirho(&["stm"]),
            ),
            ("GHC.Conc", package_exposure_chirho(&["base"])),
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: exposure_chirho.clone(),
                name_chirho: sub_mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Exception
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throw",
            "throwIO",
            "catch",
            "handle",
            "try",
            "evaluate",
            "bracket",
            "bracket_",
            "finally",
            "onException",
            "throwTo",
            "mask",
            "mask_",
            "uninterruptibleMask",
            "assert",
            "mapException",
            "displayException",
            "toException",
            "fromException",
            "catches",
            "Handler",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Exception",
            "SomeException",
            "IOException",
            "ErrorCall",
            "ArithException",
            "ArrayException",
            "AsyncException",
            "NonTermination",
            "BlockedIndefinitelyOnMVar",
            "BlockedIndefinitelyOnSTM",
            "Deadlock",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Control.Exception".to_string(),
            exports_chirho,
        });
    }

    // Control.Parallel.Strategies
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "rpar",
            "rseq",
            "rdeepseq",
            "parMap",
            "parList",
            "parListChunk",
            "dot",
            "using",
            "withStrategy",
            "evalList",
            "evalTraversable",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Strategy", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Eval", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.Parallel.Strategies".to_string(),
            exports_chirho,
        });
    }

    // Control.Exception (base)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "throw",
            "throwIO",
            "catch",
            "try",
            "evaluate",
            "bracket",
            "bracket_",
            "finally",
            "onException",
            "handle",
            "handleJust",
            "tryJust",
            "catches",
            "mask",
            "mask_",
            "uninterruptibleMask",
            "assert",
            "throwTo",
            "ioError",
            "userError",
            "catchJust",
            "Handler",
            "mapException",
            "toException",
            "fromException",
            "displayException",
            "SomeException",
            "Exception",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        // Exception class with methods — (..) import brings these in
        exports_chirho.types_chirho.insert(
            "Exception".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Exception".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "toException".to_string(),
                    "fromException".to_string(),
                    "displayException".to_string(),
                ],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        let (k_chirho, v_chirho) = mk_type_chirho("SomeException", &["SomeException"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("IOException", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho(
            "AsyncException",
            &[
                "StackOverflow",
                "HeapOverflow",
                "ThreadKilled",
                "UserInterrupt",
            ],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Control.Exception", "Control.Exception.Base"] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: package_exposure_chirho(&["base"]),
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Concurrent.STM / Control.Monad.STM
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "atomically",
            "retry",
            "orElse",
            "throwSTM",
            "catchSTM",
            "newTVar",
            "readTVar",
            "writeTVar",
            "modifyTVar",
            "newTMVar",
            "readTMVar",
            "putTMVar",
            "takeTMVar",
            "newTChan",
            "readTChan",
            "writeTChan",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STM", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["stm"]),
            name_chirho: "Control.Concurrent.STM".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["stm"]),
            name_chirho: "Control.Monad.STM".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "forkIO",
            "forkOS",
            "killThread",
            "throwTo",
            "threadDelay",
            "myThreadId",
            "yield",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ThreadId", "MVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Control.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newMVar",
            "newEmptyMVar",
            "takeMVar",
            "putMVar",
            "readMVar",
            "swapMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "isEmptyMVar",
            "withMVar",
            "modifyMVar",
            "modifyMVar_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Control.Concurrent.MVar".to_string(),
            exports_chirho,
        });
    }

    // Control.DeepSeq
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["deepseq", "force", "rnf", "rwhnf", "($!!)"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("NFData", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["deepseq"]),
            name_chirho: "Control.DeepSeq".to_string(),
            exports_chirho,
        });
    }

    // GHC.Conc — concurrent primitives
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "forkIO",
            "killThread",
            "threadDelay",
            "myThreadId",
            "STM",
            "atomically",
            "retry",
            "orElse",
            "TVar",
            "newTVar",
            "readTVar",
            "writeTVar",
            "newTVarIO",
            "readTVarIO",
            "throwSTM",
            "catchSTM",
            "labelThread",
            "forkFinally",
            "mkWeakThreadId",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STM", "TVar", "ThreadId", "ThreadStatus", "BlockReason"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["GHC.Conc", "GHC.Conc.Sync"] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: package_exposure_chirho(&["base"]),
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Concurrent.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newMVar",
            "newEmptyMVar",
            "takeMVar",
            "putMVar",
            "readMVar",
            "swapMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "isEmptyMVar",
            "withMVar",
            "modifyMVar",
            "modifyMVar_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Control.Concurrent.MVar".to_string(),
            exports_chirho,
        });
    }

    // Control.DeepSeq
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("NFData", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["deepseq", "rnf", "force", "($!!)", "NFData"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["deepseq"]),
            name_chirho: "Control.DeepSeq".to_string(),
            exports_chirho,
        });
    }

    // Control.Exception.Context
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["addExceptionContext", "getExceptionAnnotations"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"ExceptionContext";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["ExceptionContext"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Control.Exception.Context".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["ThreadId", "MVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "forkIO",
            "forkOS",
            "killThread",
            "throwTo",
            "threadDelay",
            "myThreadId",
            "newMVar",
            "readMVar",
            "takeMVar",
            "putMVar",
            "newEmptyMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "modifyMVar",
            "modifyMVar_",
            "withMVar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Control.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent / Control.Concurrent.MVar
    for mod_name_chirho in &[
        "Control.Concurrent",
        "Control.Concurrent.MVar",
        "Control.Concurrent.Chan",
        "Control.Concurrent.QSem",
        "Control.Concurrent.QSemN",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ThreadId",
            "myThreadId",
            "forkIO",
            "forkFinally",
            "forkOS",
            "killThread",
            "throwTo",
            "threadDelay",
            "threadWaitRead",
            "threadWaitWrite",
            "yield",
            "MVar",
            "newMVar",
            "newEmptyMVar",
            "readMVar",
            "takeMVar",
            "putMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "modifyMVar",
            "modifyMVar_",
            "swapMVar",
            "withMVar",
            "Chan",
            "newChan",
            "writeChan",
            "readChan",
            "dupChan",
            "getChanContents",
            "writeList2Chan",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ThreadId", "MVar", "Chan"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
}
