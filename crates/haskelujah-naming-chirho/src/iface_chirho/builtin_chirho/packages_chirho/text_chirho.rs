// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `text`, `binary`, `aeson` and `attoparsec`. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `text`, `binary`, `aeson` and `attoparsec`.
pub(super) fn push_text_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Aeson + related encoding/types modules
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "encode",
            "decode",
            "decode'",
            "eitherDecode",
            "eitherDecode'",
            "toJSON",
            "fromJSON",
            "toEncoding",
            "parseJSON",
            "encodingToLazyByteString",
            "pairs",
            "pair",
            "object",
            "withObject",
            "withArray",
            "withText",
            "withScientific",
            "withBool",
            "parseField",
            "parseFieldMaybe",
            "parseFieldMaybe'",
            "genericToJSON",
            "genericToEncoding",
            "genericParseJSON",
            "defaultOptions",
            "fieldLabelModifier",
            "constructorTagModifier",
            "allNullaryToStringTag",
            "omitNothingFields",
            "sumEncoding",
            "unwrapUnaryRecords",
            "tagSingleConstructors",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for ty_chirho in &[
            (
                "Value",
                &["Object", "Array", "String", "Number", "Bool", "Null"][..],
            ),
            ("ToJSON", &[]),
            ("FromJSON", &[]),
            ("ToJSON1", &[]),
            ("FromJSON1", &[]),
            ("Encoding", &[]),
            ("Series", &[]),
            ("Options", &["Options"]),
            (
                "SumEncoding",
                &[
                    "TaggedObject",
                    "UntaggedValue",
                    "ObjectWithSingleField",
                    "TwoElemArray",
                ],
            ),
            ("Result", &["Success", "Error"]),
            ("Parser", &[]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(ty_chirho.0, ty_chirho.1);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Share across all aeson sub-modules
        let aeson_modules_chirho = [
            "Data.Aeson",
            "Data.Aeson.Types",
            "Data.Aeson.Encoding",
            "Data.Aeson.Encoding.Internal",
            "Data.Aeson.Types.ToJSON",
            "Data.Aeson.Types.FromJSON",
            "Data.Aeson.Types.Internal",
            "Data.Aeson.Types.Generic",
            "Data.Aeson.Internal",
            "Data.Aeson.Key",
            "Data.Aeson.KeyMap",
        ];
        for mod_name_chirho in &aeson_modules_chirho {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Text.Array
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "aBA",
            "maBA",
            "empty",
            "new",
            "run",
            "copyI",
            "unsafeFreeze",
            "unsafeIndex",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (name_chirho, constructors_chirho) in &[
            ("Array", vec![]),
            ("MArray", vec![]),
            ("ByteArray", vec!["ByteArray"]),
            ("MutableByteArray", vec!["MutableByteArray"]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, constructors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Array".to_string(),
            exports_chirho,
        });
    }

    // Data.Text / Data.Text.Encoding
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pack",
            "unpack",
            "singleton",
            "empty",
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
            "intercalate",
            "intersperse",
            "transpose",
            "reverse",
            "replace",
            "toLower",
            "toUpper",
            "toTitle",
            "strip",
            "stripStart",
            "stripEnd",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "words",
            "unwords",
            "lines",
            "unlines",
            "splitOn",
            "splitAt",
            "take",
            "drop",
            "takeWhile",
            "dropWhile",
            "filter",
            "find",
            "partition",
            "index",
            "breakOn",
            "uncons",
            "concatMap",
            "any",
            "all",
            "foldr",
            "foldl",
            "foldl'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Text", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let mut enc_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["encodeUtf8", "decodeUtf8", "decodeUtf8'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            enc_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Encoding".to_string(),
            exports_chirho: enc_exports_chirho,
        });
    }

    // Data.Text.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "text", "textP", "safe", "empty", "append", "firstf", "mul", "mul32", "mul64",
            "showText", "pack",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (name_chirho, constructors_chirho) in &[("Text", vec!["Text"]), ("StrictText", vec![])]
        {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, constructors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.Text
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Text",
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
            "intercalate",
            "intersperse",
            "transpose",
            "reverse",
            "toLower",
            "toUpper",
            "toTitle",
            "toCaseFold",
            "foldl",
            "foldl'",
            "foldr",
            "concat",
            "concatMap",
            "any",
            "all",
            "maximum",
            "minimum",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "strip",
            "stripStart",
            "stripEnd",
            "words",
            "unwords",
            "lines",
            "unlines",
            "uncons",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "replace",
            "breakOn",
            "splitOn",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Text", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Text",
            "pack",
            "unpack",
            "empty",
            "fromStrict",
            "toStrict",
            "uncons",
            "null",
            "length",
            "map",
            "toCaseFold",
            "intercalate",
            "concat",
            "foldrChunks",
            "take",
            "drop",
            "splitAt",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Text", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["readFile", "writeFile", "putStr", "putStrLn", "getContents"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.IO".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.Lazy.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["readFile", "writeFile", "putStr", "putStrLn", "getContents"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy.IO".to_string(),
            exports_chirho,
        });
    }

    // Data.Binary
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["encode", "decode", "encodeFile", "decodeFile", "put", "get"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Binary";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Binary".to_string(),
            exports_chirho,
        });
    }

    // Data.Binary.Put
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Put",
            "PutM",
            "runPut",
            "putWord8",
            "putWord16be",
            "putWord16le",
            "putWord32be",
            "putWord32le",
            "putWord64be",
            "putWord64le",
            "putInt8",
            "putInt16be",
            "putInt16le",
            "putInt32be",
            "putInt32le",
            "putInt64be",
            "putInt64le",
            "putByteString",
            "putLazyByteString",
            "putShortByteString",
            "putBuilder",
            "execPut",
            "flush",
            "putDoublebe",
            "putDoublele",
            "putFloatbe",
            "putFloatle",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Put", "PutM"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Binary.Put".to_string(),
            exports_chirho,
        });
    }

    // Data.Binary.Get
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Get",
            "runGet",
            "runGetOrFail",
            "getWord8",
            "getWord16be",
            "getWord16le",
            "getWord32be",
            "getWord32le",
            "getWord64be",
            "getWord64le",
            "getInt8",
            "getInt16be",
            "getInt16le",
            "getInt32be",
            "getInt32le",
            "getInt64be",
            "getInt64le",
            "getByteString",
            "getLazyByteString",
            "getLazyByteStringNul",
            "getRemainingLazyByteString",
            "isEmpty",
            "skip",
            "bytesRead",
            "isolate",
            "lookAhead",
            "lookAheadM",
            "lookAheadE",
            "label",
            "getDoublebe",
            "getDoublele",
            "getFloatbe",
            "getFloatle",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Get";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Binary.Get".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.Lazy.Builder / Data.Text.Lazy.Builder.RealFloat (needed by scientific)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fromText",
            "fromString",
            "fromLazyText",
            "singleton",
            "toLazyText",
            "toLazyTextWith",
            "flush",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Builder", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy.Builder".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["formatRealFloat", "realFloat"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("FPFormat", &["Exponent", "Fixed", "Generic"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy.Builder.RealFloat".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["decimal", "hexadecimal"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Lazy.Builder.Int".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "inlineInterleaveST",
            "inlinePerformIO",
            "unsafeDupablePerformIO",
            "iter",
            "iterArray",
            "iter_",
            "reverseIter",
            "reverseIterArray",
            "reverseIter_",
            "unsafeHead",
            "unsafeTail",
            "lengthWord8",
            "lengthWord16",
            "takeWord8",
            "dropWord8",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Iter", &["Iter"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Unsafe".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("utf8LengthByLeader");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Internal.Encoding.Utf8".to_string(),
            exports_chirho,
        });
    }

    // Attoparsec buffer modules
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "buffer",
            "unbuffer",
            "pappend",
            "length",
            "unsafeIndex",
            "substring",
            "unsafeDrop",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Buffer", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Attoparsec.ByteString.Buffer".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "buffer",
            "unbuffer",
            "unbufferAt",
            "length",
            "pappend",
            "iter",
            "iter_",
            "substring",
            "lengthCodeUnits",
            "dropCodeUnits",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Buffer", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Attoparsec.Text.Buffer".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "fromList",
            "set",
            "memberChar",
            "memberWord8",
            "fromSet",
            "charClass",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("FastSet", &["Sorted", "Table"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Attoparsec.ByteString.FastSet".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["withPS", "mkPS"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Attoparsec.Internal.Compat".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("inlinePerformIO");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Attoparsec.Internal.Fhthagn".to_string(),
            exports_chirho,
        });
    }

    // Data.CaseInsensitive / GHC.Storable / Network.Socket* / System.TimeManager (needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mk", "original", "foldCase", "foldedCase"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("CI", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.CaseInsensitive".to_string(),
            exports_chirho,
        });
    }

    // Data.Text.Builder.Linear stub
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Builder",
            "runBuilder",
            "fromText",
            "fromChar",
            "fromString",
            "fromDec",
            "flush",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Text.Builder.Linear".to_string(),
            exports_chirho,
        });
    }
}
