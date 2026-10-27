// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: arrays, vectors, hashing and primitive memory. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for arrays, vectors, hashing and primitive memory.
pub(super) fn push_arrays_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Ix
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["range", "index", "inRange", "rangeSize"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Ix", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ix".to_string(),
            exports_chirho,
        });
    }

    // Data.Array / Data.Array.Unboxed
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "array",
            "listArray",
            "accumArray",
            "ixmap",
            "elems",
            "assocs",
            "indices",
            "bounds",
            "!",
            "//",
            "range",
            "index",
            "inRange",
            "rangeSize",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Array", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Ix", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let (k_chirho, v_chirho) = mk_type_chirho("UArray", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.Unboxed".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.IArray".to_string(),
            exports_chirho,
        });
    }

    // Data.Array.Byte
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ByteArray", &["ByteArray"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MutableByteArray", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.Byte".to_string(),
            exports_chirho,
        });
    }

    // Data.Primitive.Array / Data.Primitive
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newArray",
            "readArray",
            "writeArray",
            "indexArray",
            "sizeofArray",
            "copyArray",
            "cloneArray",
            "freezeArray",
            "thawArray",
            "runArray",
            "createArray",
            "unsafeFreezeArray",
            "unsafeThawArray",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Array", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MutableArray", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Primitive.Array".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Primitive".to_string(),
            exports_chirho,
        });
    }

    // Data.Hashable / Data.Hashable.Lifted
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["hashWithSalt", "hash", "hashUsing"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Hashable".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let mut lifted_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["liftHashWithSalt", "hashWithSalt1", "hashWithSalt2"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            lifted_exports_chirho
                .values_chirho
                .insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable1", &[]);
        lifted_exports_chirho
            .types_chirho
            .insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable2", &[]);
        lifted_exports_chirho
            .types_chirho
            .insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Hashable.Lifted".to_string(),
            exports_chirho: lifted_exports_chirho,
        });
    }

    // Data.HashMap.Strict / Data.HashMap.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "HashMap",
            "empty",
            "singleton",
            "insert",
            "lookup",
            "member",
            "delete",
            "fromList",
            "toList",
            "size",
            "null",
            "union",
            "unionWith",
            "intersection",
            "difference",
            "map",
            "filter",
            "foldlWithKey'",
            "foldrWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("HashMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashMap.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashMap.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.HashSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "HashSet",
            "empty",
            "singleton",
            "insert",
            "member",
            "delete",
            "fromList",
            "toList",
            "size",
            "null",
            "union",
            "intersection",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("HashSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashSet".to_string(),
            exports_chirho,
        });
    }

    // Data.Vector
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Vector",
            "empty",
            "singleton",
            "fromList",
            "toList",
            "length",
            "null",
            "head",
            "tail",
            "last",
            "init",
            "map",
            "filter",
            "foldl",
            "foldl'",
            "foldr",
            "take",
            "drop",
            "slice",
            "zip",
            "zipWith",
            "generate",
            "replicate",
            "cons",
            "snoc",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Vector", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Vector".to_string(),
            exports_chirho,
        });
    }

    // Data.Array
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Array",
            "array",
            "listArray",
            "accumArray",
            "bounds",
            "indices",
            "elems",
            "assocs",
            "ixmap",
            "amap",
            "Ix",
            "range",
            "index",
            "inRange",
            "rangeSize",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Array", "Ix"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array".to_string(),
            exports_chirho,
        });
    }

    // Data.HashMap.Strict (from unordered-containers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HashMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey'",
            "foldrWithKey",
            "toList",
            "fromList",
            "keys",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashMap.Strict".to_string(),
            exports_chirho,
        });
    }

    // Data.HashSet (from unordered-containers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HashSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "filter",
            "foldl'",
            "foldr",
            "toList",
            "fromList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.HashSet".to_string(),
            exports_chirho,
        });
    }

    // Data.Hashable (from hashable)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Hashable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["hash", "hashWithSalt", "hashUsing"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Hashable".to_string(),
            exports_chirho,
        });
    }

    // Data.Array.MArray
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newArray",
            "newArray_",
            "readArray",
            "writeArray",
            "getBounds",
            "getElems",
            "getAssocs",
            "freeze",
            "thaw",
            "mapArray",
            "mapIndices",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"MArray";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.MArray".to_string(),
            exports_chirho,
        });
    }

    // Data.Array.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"runSTArray";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"STArray";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Array.ST".to_string(),
            exports_chirho,
        });
    }

    // Data.Vector.Generic
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Vector",
            "Mutable",
            "length",
            "null",
            "empty",
            "singleton",
            "replicate",
            "generate",
            "cons",
            "snoc",
            "head",
            "last",
            "tail",
            "init",
            "slice",
            "take",
            "drop",
            "splitAt",
            "map",
            "imap",
            "concatMap",
            "mapM",
            "mapM_",
            "forM",
            "forM_",
            "filter",
            "ifilter",
            "takeWhile",
            "dropWhile",
            "foldl",
            "foldl'",
            "foldl1",
            "foldl1'",
            "foldr",
            "foldr'",
            "foldr1",
            "foldr1'",
            "ifoldl",
            "ifoldl'",
            "ifoldr",
            "ifoldr'",
            "all",
            "any",
            "sum",
            "product",
            "maximum",
            "minimum",
            "zip",
            "zipWith",
            "unzip",
            "elem",
            "notElem",
            "find",
            "findIndex",
            "findIndices",
            "elemIndex",
            "toList",
            "fromList",
            "fromListN",
            "convert",
            "freeze",
            "thaw",
            "copy",
            "unsafeFreeze",
            "unsafeThaw",
            "basicUnsafeFreeze",
            "basicUnsafeThaw",
            "basicLength",
            "basicUnsafeSlice",
            "basicUnsafeIndexM",
            "basicUnsafeCopy",
            "basicOverlaps",
            "basicInitialize",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Vector";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Vector.Generic".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Vector.Generic.Base".to_string(),
            exports_chirho,
        });
    }

    // Data.Array / Data.Array.IArray / Data.Array.MArray / Data.Array.ST / Data.Array.IO / Data.Array.Unboxed
    for mod_name_chirho in &[
        "Data.Array",
        "Data.Array.IArray",
        "Data.Array.MArray",
        "Data.Array.ST",
        "Data.Array.IO",
        "Data.Array.Unboxed",
        "Data.Array.Base",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Array",
            "array",
            "listArray",
            "accumArray",
            "(!)",
            "bounds",
            "indices",
            "elems",
            "assocs",
            "ixmap",
            "accum",
            "amap",
            "//",
            "range",
            "index",
            "inRange",
            "rangeSize",
            "Ix",
            "unsafeAt",
            "MArray",
            "IOArray",
            "STArray",
            "IOUArray",
            "STUArray",
            "newArray",
            "newArray_",
            "newListArray",
            "readArray",
            "writeArray",
            "getElems",
            "getBounds",
            "freeze",
            "thaw",
            "IArray",
            "UArray",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Array", "Ix", "IArray", "MArray", "IOArray", "STArray", "IOUArray", "STUArray",
            "UArray",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
}
