// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: GHC's IO, concurrency, pointer and runtime modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for GHC's IO, concurrency, pointer and runtime modules.
pub(super) fn push_io_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // GHC.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "runSTRep"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ST".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
            "stToIO",
            "ioToST",
            "throwIO",
            "catchException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["IO", "MVar", "IORef"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO".to_string(),
            exports_chirho,
        });
    }

    // GHC.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IORef",
            "newIORef",
            "readIORef",
            "writeIORef",
            "modifyIORef",
            "atomicModifyIORef",
            "atomicModifyIORef2Lazy",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &["IORef"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IORef".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "StablePtr",
            "newStablePtr",
            "deRefStablePtr",
            "freeStablePtr",
            "castStablePtrToPtr",
            "castPtrToStablePtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("StablePtr", &["StablePtr"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Stable".to_string(),
            exports_chirho,
        });
    }

    // GHC.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newMVar",
            "newEmptyMVar",
            "takeMVar",
            "putMVar",
            "readMVar",
            "tryTakeMVar",
            "tryPutMVar",
            "isEmptyMVar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.MVar".to_string(),
            exports_chirho,
        });
    }

    // GHC.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Ptr", "FunPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["nullPtr", "plusPtr", "castPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Ptr".to_string(),
            exports_chirho,
        });
    }

    // GHC.Clock
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["getMonotonicTime", "getMonotonicTimeNSec"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Clock".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.ForeignPtr (additional exports) ───────────────────────────
    // Already have Foreign.ForeignPtr, add GHC.ForeignPtr.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "ForeignPtr",
            "newForeignPtr",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "finalizeForeignPtr",
            "castForeignPtr",
            "plusForeignPtr",
            "mallocPlainForeignPtrBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ForeignPtr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ForeignPtr.Internal".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.IO.Exception ─────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "IOException",
            "IOError",
            "ioError",
            "userError",
            "BlockedIndefinitelyOnMVar",
            "BlockedIndefinitelyOnSTM",
            "AsyncException",
            "SomeAsyncException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in [
            "IOException",
            "IOError",
            "AsyncException",
            "SomeAsyncException",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO.Exception".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.Arr ──────────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Array",
            "array",
            "listArray",
            "accumArray",
            "elems",
            "indices",
            "assocs",
            "bounds",
            "(!)",
            "(//)",
            "accum",
            "ixmap",
            "range",
            "index",
            "inRange",
            "rangeSize",
            "Ix",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Array", "Ix"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Arr".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO (internal module needed by some tests)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IO", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "unsafePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO".to_string(),
            exports_chirho,
        });
    }

    // GHC.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["STRef", "newSTRef", "readSTRef", "writeSTRef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &["STRef"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.STRef".to_string(),
            exports_chirho,
        });
    }

    // GHC.IOArray
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IOArray",
            "newIOArray",
            "readIOArray",
            "writeIOArray",
            "boundsIOArray",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IOArray", &["IOArray"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IOArray".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO / GHC.IO.Handle / GHC.IO.Handle.Types
    for mod_name_chirho in &[
        "GHC.IO",
        "GHC.IO.Handle",
        "GHC.IO.Handle.Types",
        "GHC.IO.Handle.FD",
        "GHC.IO.Handle.Text",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IO",
            "Handle",
            "stdin",
            "stdout",
            "stderr",
            "HandleType",
            "HandlePosition",
            "hClose",
            "hFlush",
            "hGetContents",
            "hSetBuffering",
            "hGetBuffering",
            "hSetEncoding",
            "hSetNewlineMode",
            "BufferMode",
            "NoBuffering",
            "LineBuffering",
            "BlockBuffering",
            "NewlineMode",
            "universalNewlineMode",
            "nativeNewlineMode",
            "noNewlineTranslation",
            "Newline",
            "LF",
            "CRLF",
            "IOMode",
            "ReadMode",
            "WriteMode",
            "AppendMode",
            "ReadWriteMode",
            "openFile",
            "withFile",
            "hPutStr",
            "hPutStrLn",
            "hGetLine",
            "hGetChar",
            "hPutChar",
            "hPrint",
            "hIsEOF",
            "hIsTerminalDevice",
            "hSeek",
            "SeekMode",
            "AbsoluteSeek",
            "RelativeSeek",
            "SeekFromEnd",
            "hTell",
            "hFileSize",
            "hSetFileSize",
            "hIsOpen",
            "hIsClosed",
            "hIsReadable",
            "hIsWritable",
            "hIsSeekable",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Handle",
            "HandleType",
            "BufferMode",
            "IOMode",
            "SeekMode",
            "Newline",
            "NewlineMode",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // GHC.IO.Encoding.Failure
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CodingFailureMode",
            "ErrorOnCodingFailure",
            "IgnoreCodingFailure",
            "TransliterateCodingFailure",
            "RoundtripFailure",
            "isSurrogate",
            "recoverDecode",
            "recoverEncode",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho(
            "CodingFailureMode",
            &[
                "ErrorOnCodingFailure",
                "IgnoreCodingFailure",
                "TransliterateCodingFailure",
                "RoundtripFailure",
            ],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO.Encoding.Failure".to_string(),
            exports_chirho,
        });
    }

    // GHC.Foreign (needed by filepath OsPath modules)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "peekCStringLen",
            "newCStringLen",
            "withCStringLen",
            "withCString",
            "peekCString",
            "newCString",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Foreign".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("writeWord8OffPtr");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Storable".to_string(),
            exports_chirho,
        });
    }

    // GHC.Event — timer/event manager (needed by auto-update/warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getSystemTimerManager",
            "registerTimeout",
            "unregisterTimeout",
            "updateTimeout",
            "getSystemEventManager",
            "getSystemTimerManager",
            "registerFd",
            "unregisterFd",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["TimerManager", "TimeoutKey", "EventManager", "FdKey"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Event".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Event.TimerManager".to_string(),
            exports_chirho,
        });
    }

    // GHC.Compact
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "compact",
            "compactAdd",
            "compactAddWithSharing",
            "compactSize",
            "getCompact",
            "isCompact",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Compact", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Compact".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Compact.Serialized".to_string(),
            exports_chirho,
        });
    }

    // GHC internal modules needed by base-orphans
    for mod_name_chirho in &[
        "GHC.GHCi",
        "GHC.IO.Buffer",
        "GHC.IO.BufferedIO",
        "GHC.IO.Device",
        "GHC.Stats",
        "GHC.RTS.Flags",
        "GHC.Conc.Signal",
        "System.Console.GetOpt",
        "Text.Read.Lex",
    ] {
        let exports_chirho = IfaceExportsChirho::default();
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // GHC.Desugar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["AnnotationWrapper", "toAnnotationWrapper"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("AnnotationWrapper", &["AnnotationWrapper"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Desugar".to_string(),
            exports_chirho,
        });
    }

    // GHC.Weak
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Weak",
            "mkWeak",
            "deRefWeak",
            "finalize",
            "mkWeakPtr",
            "mkWeakPair",
            "addFinalizer",
            "mkWeakIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Weak", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Weak".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Mem.Weak".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO.Encoding / GHC.IO.Encoding.UTF8
    for mod_name_chirho in &[
        "GHC.IO.Encoding",
        "GHC.IO.Encoding.UTF8",
        "GHC.IO.Encoding.Latin1",
        "GHC.IO.Encoding.CodePage",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "TextEncoding",
            "utf8",
            "utf8_bom",
            "utf16",
            "utf16le",
            "utf16be",
            "utf32",
            "utf32le",
            "utf32be",
            "latin1",
            "char8",
            "mkTextEncoding",
            "localeEncoding",
            "getLocaleEncoding",
            "setLocaleEncoding",
            "getFileSystemEncoding",
            "setFileSystemEncoding",
            "getForeignEncoding",
            "setForeignEncoding",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("TextEncoding", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // GHC.IO.FD / GHC.IO.Handle stubs
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["FD", "fdFD", "openFile", "mkFD", "closeFD"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "GHC.IO.FD",
            "GHC.IO.Handle",
            "GHC.IO.Handle.FD",
            "GHC.IO.Handle.Types",
            "GHC.IO.Handle.Internals",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }
}
