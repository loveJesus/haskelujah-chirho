// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `GHC.Prim` and the other primitive modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::mk_data_type_chirho;
use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};
use haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho;

/// Push the interfaces for `GHC.Prim` and the other primitive modules.
pub(super) fn push_prim_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // GHC.CString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "unpackCString#",
            "unpackAppendCString#",
            "unpackFoldrCString#",
            "unpackCStringUtf8#",
            "unpackAppendCStringUtf8#",
            "unpackFoldrCStringUtf8#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.CString".to_string(),
            exports_chirho,
        });
    }

    // GHC.ByteOrder
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["LittleEndian", "BigEndian", "targetByteOrder"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ByteOrder", &["LittleEndian", "BigEndian"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ByteOrder".to_string(),
            exports_chirho,
        });
    }

    // GHC.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["RuntimeRep", "Levity", "Multiplicity", "Symbol"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for constructor_chirho in [
            CompilerTypeConstructorChirho::TypeUniverseChirho,
            CompilerTypeConstructorChirho::ConstraintUniverseChirho,
        ] {
            let (key_chirho, type_chirho) = mk_data_type_chirho(constructor_chirho, &[]);
            exports_chirho.types_chirho.insert(key_chirho, type_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Types".to_string(),
            exports_chirho,
        });
    }

    // GHC.Tuple
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Solo", "Unit", "Tuple0", "Tuple1", "Tuple2", "Tuple3", "Tuple4", "Tuple5", "Tuple6",
            "Tuple7",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_val_chirho("Solo");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Tuple".to_string(),
            exports_chirho,
        });
    }

    // GHC.Classes
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "==", "/=", "<", "<=", ">", ">=", "compare", "max", "min", "not", "&&", "||",
            "divInt#", "modInt#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Eq", "Ord", "IP"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Classes".to_string(),
            exports_chirho,
        });
    }

    // GHC.Prim
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Int#",
            "Word#",
            "Float#",
            "Double#",
            "Char#",
            "Addr#",
            "MutableByteArray#",
            "ByteArray#",
            "Array#",
            "MutableArray#",
            "SmallArray#",
            "SmallMutableArray#",
            "MutVar#",
            "TVar#",
            "MVar#",
            "State#",
            "RealWorld",
            "Weak#",
            "StableName#",
            "StablePtr#",
            "Proxy#",
            "TYPE",
            "RuntimeRep",
            "LiftedRep",
            "UnliftedRep",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "seq",
            "realWorld#",
            "proxy#",
            "void#",
            "coerce",
            // Boxed type constructors (data constructors for unboxed types)
            "I#",
            "W#",
            "D#",
            "F#",
            "C#",
            // Arithmetic primops
            "+#",
            "-#",
            "*#",
            "negateInt#",
            "quotInt#",
            "remInt#",
            "+##",
            "-##",
            "*##",
            "/##",
            "==##",
            "/=##",
            "<##",
            "<=##",
            ">##",
            ">=##",
            "negateDouble#",
            "plusFloat#",
            "minusFloat#",
            "timesFloat#",
            "divideFloat#",
            "negateFloat#",
            "plusWord#",
            "minusWord#",
            "timesWord#",
            "timesWord2#",
            "quotWord#",
            "remWord#",
            // Comparison primops
            ">#",
            ">=#",
            "==#",
            "/=#",
            "<#",
            "<=#",
            "gtWord#",
            "geWord#",
            "eqWord#",
            "neWord#",
            "ltWord#",
            "leWord#",
            // Bitwise / shift primops
            "and#",
            "or#",
            "xor#",
            "not#",
            "iShiftL#",
            "iShiftRA#",
            "iShiftRL#",
            "keepAlive#",
            "uncheckedIShiftL#",
            "uncheckedIShiftRA#",
            "uncheckedIShiftRL#",
            "uncheckedShiftL#",
            "uncheckedShiftRL#",
            "popCnt#",
            "popCnt8#",
            "popCnt16#",
            "popCnt32#",
            "popCnt64#",
            "clz#",
            "clz8#",
            "clz16#",
            "clz32#",
            "clz64#",
            "ctz#",
            "ctz8#",
            "ctz16#",
            "ctz32#",
            "ctz64#",
            "byteSwap#",
            "byteSwap16#",
            "byteSwap32#",
            "byteSwap64#",
            "narrow8Int#",
            "narrow16Int#",
            "narrow32Int#",
            "narrow8Word#",
            "narrow16Word#",
            "narrow32Word#",
            // Conversion primops
            "int2Word#",
            "word2Int#",
            "int2Double#",
            "double2Int#",
            "int2Float#",
            "float2Int#",
            "float2Double#",
            "double2Float#",
            "chr#",
            "ord#",
            "word2Double#",
            "word2Float#",
            "plusAddr#",
            "minusAddr#",
            // Array primops
            "newArray#",
            "readArray#",
            "writeArray#",
            "indexArray#",
            "freezeArray#",
            "thawArray#",
            "copyArray#",
            "copyMutableArray#",
            "cloneArray#",
            "cloneMutableArray#",
            "sameMutableArray#",
            "sizeofArray#",
            "sizeofMutableArray#",
            "newByteArray#",
            "newPinnedByteArray#",
            "newAlignedPinnedByteArray#",
            "setByteArray#",
            "readIntArray#",
            "readWordArray#",
            "readWord8Array#",
            "readWord16Array#",
            "readWord32Array#",
            "readWord64Array#",
            "readInt8Array#",
            "readInt16Array#",
            "readInt32Array#",
            "readInt64Array#",
            "readFloatArray#",
            "readDoubleArray#",
            "readWideCharArray#",
            "readAddrArray#",
            "readStablePtrArray#",
            "writeIntArray#",
            "writeWordArray#",
            "writeWord8Array#",
            "writeWord8ArrayAsWord64#",
            "casIntArray#",
            "fetchAddIntArray#",
            "fetchSubIntArray#",
            "fetchAndIntArray#",
            "fetchNandIntArray#",
            "fetchOrIntArray#",
            "fetchXorIntArray#",
            "atomicReadIntArray#",
            "atomicWriteIntArray#",
            "writeWord16Array#",
            "writeWord32Array#",
            "writeWord64Array#",
            "writeInt8Array#",
            "writeInt16Array#",
            "writeInt32Array#",
            "writeInt64Array#",
            "writeFloatArray#",
            "writeDoubleArray#",
            "writeWideCharArray#",
            "writeAddrArray#",
            "writeStablePtrArray#",
            "indexIntArray#",
            "indexWordArray#",
            "indexWord8Array#",
            "indexWord8ArrayAsWord64#",
            "indexWord16Array#",
            "indexWord32Array#",
            "indexWord64Array#",
            "indexInt8Array#",
            "indexInt16Array#",
            "indexInt32Array#",
            "indexInt64Array#",
            "indexFloatArray#",
            "indexDoubleArray#",
            "indexWideCharArray#",
            "indexAddrArray#",
            "indexStablePtrArray#",
            "readWord8OffAddr#",
            "readWordOffAddr#",
            "readWord16OffAddr#",
            "readWord32OffAddr#",
            "readWord64OffAddr#",
            "readIntOffAddr#",
            "readInt8OffAddr#",
            "readInt16OffAddr#",
            "readInt32OffAddr#",
            "readInt64OffAddr#",
            "readFloatOffAddr#",
            "readDoubleOffAddr#",
            "readWideCharOffAddr#",
            "readAddrOffAddr#",
            "readStablePtrOffAddr#",
            "writeWord8OffAddr#",
            "writeWordOffAddr#",
            "writeWord16OffAddr#",
            "writeWord32OffAddr#",
            "writeWord64OffAddr#",
            "writeIntOffAddr#",
            "writeInt8OffAddr#",
            "writeInt16OffAddr#",
            "writeInt32OffAddr#",
            "writeInt64OffAddr#",
            "writeFloatOffAddr#",
            "writeDoubleOffAddr#",
            "writeWideCharOffAddr#",
            "writeAddrOffAddr#",
            "writeStablePtrOffAddr#",
            "indexWord8OffAddr#",
            "indexWordOffAddr#",
            "indexWord16OffAddr#",
            "indexWord32OffAddr#",
            "indexWord64OffAddr#",
            "indexIntOffAddr#",
            "indexInt8OffAddr#",
            "indexInt16OffAddr#",
            "indexInt32OffAddr#",
            "indexInt64OffAddr#",
            "indexFloatOffAddr#",
            "indexDoubleOffAddr#",
            "indexWideCharOffAddr#",
            "indexAddrOffAddr#",
            "indexStablePtrOffAddr#",
            "sizeofByteArray#",
            "sizeofMutableByteArray#",
            "getSizeofMutableByteArray#",
            "sameMutableByteArray#",
            "resizeMutableByteArray#",
            "shrinkMutableByteArray#",
            "isMutableByteArrayPinned#",
            "copyByteArray#",
            "copyMutableByteArray#",
            "copyByteArrayToAddr#",
            "copyAddrToByteArray#",
            "copyMutableByteArrayToAddr#",
            "compareByteArrays#",
            "readCharArray#",
            "writeCharArray#",
            "indexCharArray#",
            "unsafeFreezeByteArray#",
            "byteArrayContents#",
            "isByteArrayPinned#",
            "unsafeFreezeArray#",
            "unsafeThawArray#",
            // MutVar primops
            "newMutVar#",
            "readMutVar#",
            "writeMutVar#",
            "sameMutVar#",
            "atomicModifyMutVar#",
            // MVar primops
            "newMVar#",
            "takeMVar#",
            "readMVar#",
            "tryReadMVar#",
            "isEmptyMVar#",
            "sameMVar#",
            "putMVar#",
            "tryTakeMVar#",
            "tryPutMVar#",
            // Weak pointer / stable name primops
            "mkWeak#",
            "deRefWeak#",
            "finalizeWeak#",
            "makeStableName#",
            "eqStableName#",
            "stableNameToInt#",
            // Misc primops
            "dataToTag#",
            "tagToEnum#",
            "reallyUnsafePtrEquality#",
            "touch#",
            "noDuplicate#",
            "raise#",
            "raiseIO#",
            "catch#",
            "maskAsyncExceptions#",
            "unmaskAsyncExceptions#",
            "atomically#",
            "retry#",
            "catchRetry#",
            "catchSTM#",
            "newTVar#",
            "readTVar#",
            "readTVarIO#",
            "writeTVar#",
            // Comparison primops needed by word8, transformers-compat, etc.
            "isTrue#",
            "eqChar#",
            "neChar#",
            "ltChar#",
            "leChar#",
            "gtChar#",
            "geChar#",
            "eqAddr#",
            "neAddr#",
            "ltAddr#",
            "leAddr#",
            "gtAddr#",
            "geAddr#",
            "eqFloat#",
            "neFloat#",
            "ltFloat#",
            "leFloat#",
            "gtFloat#",
            "geFloat#",
            "eqDouble#",
            "neDouble#",
            "ltDouble#",
            "leDouble#",
            "gtDouble#",
            "geDouble#",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Prim".to_string(),
            exports_chirho,
        });
    }

    // GHC.Magic
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["inline", "noinline", "lazy", "oneShot", "runRW#"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Magic".to_string(),
            exports_chirho,
        });
    }
}
