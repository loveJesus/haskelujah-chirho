// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `bytestring`. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `bytestring`.
pub(super) fn push_bytestring_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.ByteString.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "c2w",
            "w2c",
            "memset",
            "memcpy",
            "plusForeignPtr",
            "unsafeCreate",
            "create",
            "createAndTrim",
            "mallocByteString",
            "nullForeignPtr",
            "accursedUnutterablePerformIO",
            "toForeignPtr",
            "fromForeignPtr",
            "unsafeHead",
            "unsafeTail",
            "unsafeLast",
            "unsafeInit",
            "unsafeIndex",
            "unsafeTake",
            "unsafeDrop",
            "packBytes",
            "unpackBytes",
            "unpackChars",
            "packChars",
            "isSpaceWord8",
            "checkedAdd",
            "mallocPlainForeignPtrBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &["BS", "PS"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Internal".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Internal.Type".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Short / Data.ByteString.Short.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toShort",
            "fromShort",
            "pack",
            "unpack",
            "empty",
            "null",
            "length",
            "index",
            "head",
            "last",
            "tail",
            "init",
            "cons",
            "snoc",
            "append",
            "isPrefixOf",
            "isSuffixOf",
            "concat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ShortByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_chirho in &["Data.ByteString.Short", "Data.ByteString.Short.Internal"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.ByteString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pack",
            "unpack",
            "empty",
            "singleton",
            "cons",
            "snoc",
            "append",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "map",
            "reverse",
            "intercalate",
            "transpose",
            "foldl",
            "foldl'",
            "foldr",
            "concat",
            "concatMap",
            "any",
            "all",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "break",
            "span",
            "filter",
            "find",
            "partition",
            "index",
            "elem",
            "notElem",
            "isPrefixOf",
            "isSuffixOf",
            "hGetSome",
            "hGetNonBlocking",
            "hGet",
            "hGetContents",
            "hPut",
            "hPutNonBlocking",
            "hPutStr",
            "readFile",
            "writeFile",
            "appendFile",
            "getLine",
            "getContents",
            "putStr",
            "putStrLn",
            "interact",
            "copy",
            "replicate",
            "unfoldr",
            "unfoldrN",
            "sort",
            "group",
            "groupBy",
            "inits",
            "tails",
            "stripPrefix",
            "stripSuffix",
            "isInfixOf",
            "zip",
            "zipWith",
            "unzip",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ByteString",
            "empty",
            "singleton",
            "pack",
            "unpack",
            "packCStringLen",
            "useAsCStringLen",
            "uncons",
            "cons",
            "snoc",
            "append",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "map",
            "reverse",
            "intercalate",
            "foldl",
            "foldl'",
            "foldr",
            "concat",
            "concatMap",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "readFile",
            "writeFile",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Char8".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unsafeUseAsCString",
            "unsafeUseAsCStringLen",
            "unsafePackCString",
            "unsafePackCStringLen",
            "unsafePackCStringFinalizer",
            "unsafePackMallocCString",
            "unsafeHead",
            "unsafeTail",
            "unsafeLast",
            "unsafeInit",
            "unsafeTake",
            "unsafeDrop",
            "unsafeIndex",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Builder
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Builder",
            "toLazyByteString",
            "hPutBuilder",
            "byteString",
            "lazyByteString",
            "shortByteString",
            "int8",
            "word8",
            "int16BE",
            "int16LE",
            "int32BE",
            "int32LE",
            "int64BE",
            "int64LE",
            "word16BE",
            "word16LE",
            "word32BE",
            "word32LE",
            "word64BE",
            "word64LE",
            "wordHost",
            "word16Host",
            "word32Host",
            "word64Host",
            "intHost",
            "int16Host",
            "int32Host",
            "int64Host",
            "floatBE",
            "floatLE",
            "doubleBE",
            "doubleLE",
            "char7",
            "string7",
            "char8",
            "string8",
            "charUtf8",
            "stringUtf8",
            "intDec",
            "integerDec",
            "int8Dec",
            "int16Dec",
            "int32Dec",
            "int64Dec",
            "wordDec",
            "word8Dec",
            "word16Dec",
            "word32Dec",
            "word64Dec",
            "intHex",
            "int8HexFixed",
            "int16HexFixed",
            "int32HexFixed",
            "int64HexFixed",
            "wordHex",
            "word8Hex",
            "word16Hex",
            "word32Hex",
            "word64Hex",
            "word8HexFixed",
            "word16HexFixed",
            "word32HexFixed",
            "word64HexFixed",
            "floatHexFixed",
            "doubleHexFixed",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Builder";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Builder".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Builder.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Builder.Extra
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toLazyByteStringWith",
            "AllocationStrategy",
            "safeStrategy",
            "untrimmedStrategy",
            "smallChunkSize",
            "defaultChunkSize",
            "byteStringCopy",
            "byteStringInsert",
            "byteStringThreshold",
            "lazyByteStringCopy",
            "lazyByteStringInsert",
            "lazyByteStringThreshold",
            "flush",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Builder.Extra".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ByteString",
            "empty",
            "pack",
            "unpack",
            "uncons",
            "fromStrict",
            "toStrict",
            "null",
            "length",
            "map",
            "concat",
            "take",
            "drop",
            "readFile",
            "writeFile",
            "getContents",
            "foldrChunks",
            "foldlChunks",
            "toChunks",
            "fromChunks",
            "foldr",
            "foldl'",
            "append",
            "intercalate",
            "hGet",
            "hPut",
            "hGetContents",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy.Char8".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Data.ByteString.Lazy.Internal
        {
            let mut internal_exports_chirho = exports_chirho.clone();
            for name_chirho in &[
                "ByteString",
                "chunk",
                "foldrChunks",
                "foldlChunks",
                "packBytes",
                "packChars",
                "unpackBytes",
                "unpackChars",
                "smallChunkSize",
                "defaultChunkSize",
                "Chunk",
                "Empty",
                "invariant",
                "checkInvariant",
                "unsafeHead",
                "unsafeTail",
            ] {
                let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
                internal_exports_chirho
                    .values_chirho
                    .insert(k_chirho, v_chirho);
            }
            let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &["Chunk", "Empty"]);
            internal_exports_chirho
                .types_chirho
                .insert(k_chirho, v_chirho);
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: "Data.ByteString.Lazy.Internal".to_string(),
                exports_chirho: internal_exports_chirho,
            });
        }
    }

    // Data.ByteString.Builder.Prim
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "BoundedPrim",
            "FixedPrim",
            "primFixed",
            "primBounded",
            ">*<",
            "primMapListFixed",
            "primMapListBounded",
            "primUnfoldrFixed",
            "primUnfoldrBounded",
            "primMapByteStringFixed",
            "primMapLazyByteStringFixed",
            "word8",
            "int8",
            "word16BE",
            "word16LE",
            "word32BE",
            "word32LE",
            "word64BE",
            "word64LE",
            "int16BE",
            "int16LE",
            "int32BE",
            "int32LE",
            "int64BE",
            "int64LE",
            "floatBE",
            "floatLE",
            "doubleBE",
            "doubleLE",
            "intDec",
            "int8Dec",
            "int16Dec",
            "int32Dec",
            "int64Dec",
            "integerDec",
            "wordDec",
            "word8Dec",
            "word16Dec",
            "word32Dec",
            "word64Dec",
            "wordHex",
            "word8Hex",
            "word16Hex",
            "word32Hex",
            "word64Hex",
            "int8HexFixed",
            "int16HexFixed",
            "int32HexFixed",
            "int64HexFixed",
            "word8HexFixed",
            "word16HexFixed",
            "word32HexFixed",
            "word64HexFixed",
            "floatHexFixed",
            "doubleHexFixed",
            "char7",
            "char8",
            "charUtf8",
            "condB",
            "emptyB",
            "liftFixedToBounded",
            // Host-endian variants
            "wordHost",
            "word16Host",
            "word32Host",
            "word64Host",
            "intHost",
            "int16Host",
            "int32Host",
            "int64Host",
            "floatHost",
            "doubleHost",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["BoundedPrim", "FixedPrim"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Builder.Prim".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Builder.Prim.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.Word8 (word8 — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "_0",
            "_9",
            "_A",
            "_Z",
            "_a",
            "_z",
            "_colon",
            "_semicolon",
            "_space",
            "_tab",
            "_cr",
            "_lf",
            "_period",
            "_comma",
            "isUpper",
            "isLower",
            "isDigit",
            "isAlpha",
            "isAlphaNum",
            "isSpace",
            "toLower",
            "toUpper",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word8".to_string(),
            exports_chirho,
        });
    }

    // Data.Streaming.ByteString.Builder.Buffer (streaming-commons — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Buffer", &["Buffer"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Buffer");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Streaming.ByteString.Builder.Buffer".to_string(),
            exports_chirho,
        });
    }

    // Data.ByteString.UTF8 (utf8-string package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fromString",
            "toString",
            "decode",
            "encode",
            "length",
            "lines",
            "unlines",
            "words",
            "unwords",
            "take",
            "drop",
            "span",
            "break",
            "splitAt",
            "foldl",
            "foldr",
            "null",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.UTF8".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.ByteString.Lazy.UTF8".to_string(),
            exports_chirho,
        });
    }
}
