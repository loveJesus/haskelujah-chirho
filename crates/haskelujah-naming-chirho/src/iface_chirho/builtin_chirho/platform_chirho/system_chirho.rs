// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `System.*` IO, process and environment modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `System.*` IO, process and environment modules.
pub(super) fn push_system_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "putStr",
            "putStrLn",
            "print",
            "getLine",
            "getContents",
            "readFile",
            "writeFile",
            "appendFile",
            "hSetBuffering",
            "hGetBuffering",
            "hGetEcho",
            "hIsTerminalDevice",
            "hGetContents",
            "hPutStr",
            "hPutStrLn",
            "hFlush",
            "hClose",
            "hSetEncoding",
            "hSetEcho",
            "stdin",
            "stdout",
            "stderr",
            "withFile",
            "openFile",
            "openBinaryFile",
            "hSetBinaryMode",
            "hIsEOF",
            "isEOF",
            "hGetChar",
            "hGetLine",
            "hLookAhead",
            "hReady",
            "hPutChar",
            "hPrint",
            "hTell",
            "hSeek",
            "hFileSize",
            "hIsOpen",
            "hIsClosed",
            "hIsReadable",
            "hIsWritable",
            "hIsSeekable",
            "hIsTerminalDevice",
            "hSetNewlineMode",
            "hGetEncoding",
            "mkTextEncoding",
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
            "localeEncoding",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "IO",
            "Handle",
            "IOMode",
            "BufferMode",
            "SeekMode",
            "NewlineMode",
            "Newline",
            "TextEncoding",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "hFlush",
            "hPutStr",
            "hPutStrLn",
            "hGetLine",
            "hSetBuffering",
            "hGetBuffering",
            "hSetEncoding",
            "hClose",
            "openFile",
            "withFile",
            "stdin",
            "stdout",
            "stderr",
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
            "localeEncoding",
            "hGetContents",
            "hPrint",
            "putStr",
            "putStrLn",
            "print",
            "getLine",
            "getContents",
            "readFile",
            "writeFile",
            "appendFile",
            "hIsTerminalDevice",
            "openTempFile",
            "openBinaryTempFile",
            "openTempFileWithDefaultPermissions",
            "openBinaryTempFileWithDefaultPermissions",
            "hSetBinaryMode",
            "hIsEOF",
            "isEOF",
            "hSeek",
            "hTell",
            "hFileSize",
            "hReady",
            "hWaitForInput",
            "hLookAhead",
            "hGetChar",
            "hPutChar",
            "interact",
            "getChar",
            "putChar",
            "fixIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Handle", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho(
            "IOMode",
            &["ReadMode", "WriteMode", "AppendMode", "ReadWriteMode"],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho(
            "BufferMode",
            &["NoBuffering", "LineBuffering", "BlockBuffering"],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TextEncoding", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["exitWith", "exitFailure", "exitSuccess", "die"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // System.Mem.StableName
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["makeStableName", "hashStableName", "eqStableName"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("StableName", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Mem.StableName".to_string(),
            exports_chirho,
        });
    }

    // System.IO.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafePerformIO",
            "inlinePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["exitWith", "exitFailure", "exitSuccess", "die"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["ExitSuccess", "ExitFailure"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (already exists — add Data.IORef.Strict alias)
    // System.IO.Error
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IOError",
            "ioError",
            "userError",
            "mkIOError",
            "isAlreadyExistsError",
            "isDoesNotExistError",
            "isAlreadyInUseError",
            "isFullError",
            "isEOFError",
            "isIllegalOperation",
            "isPermissionError",
            "isUserError",
            "ioeGetErrorType",
            "ioeGetLocation",
            "ioeGetErrorString",
            "ioeGetHandle",
            "ioeGetFileName",
            "tryIOError",
            "catchIOError",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IOError", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Error".to_string(),
            exports_chirho,
        });
    }

    // System.IO.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafePerformIO",
            "inlinePerformIO",
            "unsafeInterleaveIO",
            "unsafeDupablePerformIO",
            "unsafeFixIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "ExitSuccess",
            "ExitFailure",
            "exitWith",
            "exitSuccess",
            "exitFailure",
            "die",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // System.Environment
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getArgs",
            "getProgName",
            "getEnv",
            "getEnvironment",
            "lookupEnv",
            "setEnv",
            "unsetEnv",
            "withArgs",
            "withProgName",
            "getExecutablePath",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Environment".to_string(),
            exports_chirho,
        });
    }

    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Handle", "IOMode", "BufferMode", "SeekMode"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "stdin",
            "stdout",
            "stderr",
            "openFile",
            "hClose",
            "hFlush",
            "hPutStr",
            "hPutStrLn",
            "hPutChar",
            "hGetLine",
            "hGetContents",
            "hGetChar",
            "hSetBuffering",
            "hSetEncoding",
            "hIsEOF",
            "isEOF",
            "withFile",
            "withBinaryFile",
            "readFile",
            "writeFile",
            "appendFile",
            "ReadMode",
            "WriteMode",
            "AppendMode",
            "ReadWriteMode",
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
            "localeEncoding",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("TextEncoding", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // System.CPUTime
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["getCPUTime", "cpuTimePrecision"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.CPUTime".to_string(),
            exports_chirho,
        });
    }

    // System.Mem
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["performGC", "performMajorGC", "performMinorGC"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Mem".to_string(),
            exports_chirho,
        });
    }

    // System.Directory (basic)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "doesFileExist",
            "doesDirectoryExist",
            "createDirectory",
            "createDirectoryIfMissing",
            "removeFile",
            "removeDirectory",
            "renameFile",
            "renameDirectory",
            "getDirectoryContents",
            "getCurrentDirectory",
            "setCurrentDirectory",
            "getHomeDirectory",
            "getTemporaryDirectory",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Directory".to_string(),
            exports_chirho,
        });
    }

    // System.Process (basic)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callProcess",
            "callCommand",
            "readProcess",
            "readProcessWithExitCode",
            "system",
            "rawSystem",
            "createProcess",
            "waitForProcess",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["CreateProcess", "ProcessHandle", "StdStream"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Process".to_string(),
            exports_chirho,
        });
    }

    // System.Timeout
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"timeout";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Timeout".to_string(),
            exports_chirho,
        });
    }

    // System.Environment
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getArgs",
            "getProgName",
            "getExecutablePath",
            "getEnvironment",
            "lookupEnv",
            "setEnv",
            "unsetEnv",
            "withArgs",
            "withProgName",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Environment".to_string(),
            exports_chirho,
        });
    }
}
