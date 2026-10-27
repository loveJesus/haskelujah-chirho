// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: POSIX, OS strings, file paths and the console. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for POSIX, OS strings, file paths and the console.
pub(super) fn push_posix_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // System.Posix.ByteString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"RawFilePath";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Posix.ByteString".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Posix.ByteString.FilePath".to_string(),
            exports_chirho,
        });
    }

    // System.Posix.Types / System.Posix.Internals
    for mod_name_chirho in &["System.Posix.Types", "System.Posix.Internals"] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Fd",
            "FileMode",
            "FileOffset",
            "ProcessID",
            "ProcessGroupID",
            "UserID",
            "GroupID",
            "ByteCount",
            "ClockTick",
            "EpochTime",
            "DeviceID",
            "FileID",
            "Limit",
            "LinkCount",
            "CDev",
            "CIno",
            "CMode",
            "COff",
            "CPid",
            "CSsize",
            "CGid",
            "CNlink",
            "CUid",
            "CCc",
            "CSpeed",
            "CTcflag",
            "CRLim",
            "CSocklen",
            "CKey",
            "CId",
            "CFsBlkCnt",
            "CFsFilCnt",
            "CClockId",
            "CBlkSize",
            "CBlkCnt",
            "fdToHandle",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Fd",
            "FileMode",
            "FileOffset",
            "ProcessID",
            "UserID",
            "GroupID",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // System.Posix / System.Posix.Directory / System.Posix.Files
    for mod_name_chirho in &[
        "System.Posix",
        "System.Posix.Directory",
        "System.Posix.Files",
        "System.Posix.IO",
        "System.Posix.Process",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "openDirStream",
            "readDirStream",
            "closeDirStream",
            "getWorkingDirectory",
            "changeWorkingDirectory",
            "createDirectory",
            "removeDirectory",
            "getFileStatus",
            "getSymbolicLinkStatus",
            "isDirectory",
            "isRegularFile",
            "fileMode",
            "intersectFileModes",
            "ownerReadMode",
            "ownerWriteMode",
            "ownerExecuteMode",
            "groupReadMode",
            "otherReadMode",
            "isSymbolicLink",
            "fileSize",
            "modificationTime",
            "accessTime",
            "createSymbolicLink",
            "readSymbolicLink",
            "rename",
            "removeLink",
            "setFileMode",
            "openFd",
            "closeFd",
            "fdRead",
            "fdWrite",
            "getProcessID",
            "epochTime",
            "forkProcess",
            "defaultFileFlags",
            "setFdOption",
            "openFd",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["DirStream", "FileStatus", "OpenFileFlags", "FdOption"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // System.OsString.Encoding.Internal (os-string — needed by filepath)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ucs2le",
            "mkUcs2le",
            "ucs2le_decode",
            "ucs2le_encode",
            "utf16le_b",
            "mkUTF16le_b",
            "utf16le_b_decode",
            "utf16le_b_encode",
            "encodeWithBasePosix",
            "decodeWithBasePosix",
            "encodeWithBaseWindows",
            "decodeWithBaseWindows",
            "showEncodingException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("EncodingException", &["EncodingError"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.OsString.Encoding.Internal".to_string(),
            exports_chirho,
        });
    }

    // System.OsString.Data.ByteString.Short (os-string — needed by filepath)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ShortByteString",
            "toShort",
            "fromShort",
            "pack",
            "unpack",
            "empty",
            "null",
            "length",
            "index",
            "append",
            "useAsCStringLen",
            "packCStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ShortByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.OsString.Data.ByteString.Short".to_string(),
            exports_chirho,
        });
    }

    // System.OsString.Internal.Types (os-string — needed by hashable/filepath)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "OsString",
            "getOsString",
            "PosixString",
            "getPosixString",
            "WindowsString",
            "getWindowsString",
            "PosixChar",
            "WindowsChar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "OsString",
            "PosixString",
            "WindowsString",
            "PosixChar",
            "WindowsChar",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.OsString.Internal.Types".to_string(),
            exports_chirho,
        });
    }

    // System.Console.ANSI.Types (ansi-terminal-types — needed by ansi-terminal)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "setSGRCode",
            "csi",
            "sgrToCode",
            "colorToCode",
            "xtermSystem",
            "xterm6LevelRGB",
            "xterm24LevelGray",
            "cursorUpCode",
            "cursorDownCode",
            "cursorForwardCode",
            "cursorBackwardCode",
            "cursorDownLineCode",
            "cursorUpLineCode",
            "setCursorColumnCode",
            "setCursorPositionCode",
            "saveCursorCode",
            "restoreCursorCode",
            "reportCursorPositionCode",
            "clearFromCursorToScreenEndCode",
            "clearFromCursorToScreenBeginningCode",
            "clearScreenCode",
            "clearFromCursorToLineEndCode",
            "clearFromCursorToLineBeginningCode",
            "clearLineCode",
            "enableLineWrapCode",
            "disableLineWrapCode",
            "scrollPageUpCode",
            "scrollPageDownCode",
            "useAlternateScreenBufferCode",
            "useNormalScreenBufferCode",
            "reportLayerColorCode",
            "hideCursorCode",
            "showCursorCode",
            "hyperlinkWithParamsCode",
            "setTitleCode",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (type_name_chirho, ctors_chirho) in &[
            (
                "SGR",
                vec![
                    "Reset",
                    "SetConsoleIntensity",
                    "SetItalicized",
                    "SetUnderlining",
                    "SetBlinkSpeed",
                    "SetVisible",
                    "SetSwapForegroundBackground",
                    "SetColor",
                    "SetRGBColor",
                    "SetPaletteColor",
                    "SetDefaultColor",
                ],
            ),
            (
                "Color",
                vec![
                    "Black", "Red", "Green", "Yellow", "Blue", "Magenta", "Cyan", "White",
                ],
            ),
            ("ColorIntensity", vec!["Dull", "Vivid"]),
            (
                "ConsoleLayer",
                vec!["Foreground", "Background", "Underlining"],
            ),
            (
                "ConsoleIntensity",
                vec!["BoldIntensity", "FaintIntensity", "NormalIntensity"],
            ),
            ("BlinkSpeed", vec!["SlowBlink", "RapidBlink", "NoBlink"]),
            (
                "Underlining",
                vec![
                    "SingleUnderline",
                    "DoubleUnderline",
                    "CurlyUnderline",
                    "DottedUnderline",
                    "DashedUnderline",
                    "NoUnderline",
                ],
            ),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_name_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["System.Console.ANSI.Types", "System.Console.ANSI.Codes"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // System.TimeManager (time-manager — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "initialize",
            "stopManager",
            "killManager",
            "withManager",
            "register",
            "tickle",
            "pause",
            "resume",
            "cancel",
            "setTimeout",
            "withHandleKillThread",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Manager", "TimeoutAction", "Handle"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.TimeManager".to_string(),
            exports_chirho,
        });
    }

    // System.FilePath.Posix / System.FilePath.Windows / System.FilePath
    for mod_name_chirho in &[
        "System.FilePath",
        "System.FilePath.Posix",
        "System.FilePath.Windows",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "FilePath",
            "pathSeparator",
            "pathSeparators",
            "isPathSeparator",
            "searchPathSeparator",
            "isSearchPathSeparator",
            "extSeparator",
            "isExtSeparator",
            "splitExtension",
            "takeExtension",
            "replaceExtension",
            "dropExtension",
            "addExtension",
            "hasExtension",
            "splitExtensions",
            "takeExtensions",
            "dropExtensions",
            "splitFileName",
            "takeFileName",
            "replaceFileName",
            "dropFileName",
            "takeBaseName",
            "replaceBaseName",
            "takeDirectory",
            "replaceDirectory",
            "combine",
            "splitPath",
            "joinPath",
            "splitDirectories",
            "splitDrive",
            "joinDrive",
            "takeDrive",
            "hasDrive",
            "dropDrive",
            "isDrive",
            "hasTrailingPathSeparator",
            "addTrailingPathSeparator",
            "dropTrailingPathSeparator",
            "normalise",
            "equalFilePath",
            "makeRelative",
            "isRelative",
            "isAbsolute",
            "isValid",
            "makeValid",
            "</>",
            "<.>",
            "-<.>",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
}
