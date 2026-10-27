// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `Data.Map` and `Data.IntMap`. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `Data.Map` and `Data.IntMap`.
pub(super) fn push_maps_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mapEmpty",
            "mapInsert",
            "mapLookup",
            "mapMember",
            "mapDelete",
            "mapFromList",
            "mapToList",
            "mapSize",
            "mapKeys",
            "mapElems",
            "mapFoldlWithKey",
            "mapInsertWith",
            "mapFindWithDefault",
            "mapAdjust",
            "mapUnionWith",
            "mapMap",
            "mapFilter",
            "mapNull",
            "mapInsertStr",
            "mapLookupStr",
            "mapMemberStr",
            "mapDeleteStr",
            "mapFindWithDefaultStr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &["MapEmpty", "MapNode"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        // Also export constructors as values
        for con_chirho in &["MapEmpty", "MapNode"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map".to_string(),
            exports_chirho,
        });
    }

    // Data.Map.Strict (alias to Data.Map)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "insertWith",
            "delete",
            "lookup",
            "member",
            "findWithDefault",
            "adjust",
            "update",
            "union",
            "unionWith",
            "unions",
            "unionsWith",
            "intersection",
            "intersectionWith",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldr",
            "foldl",
            "foldrWithKey",
            "foldlWithKey",
            "null",
            "size",
            "keys",
            "elems",
            "toList",
            "fromList",
            "fromListWith",
            "toAscList",
            "toDescList",
            "fromAscList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.Map.Merge.Lazy / Data.Map.Merge.Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "merge",
            "zipWithMaybeAMatched",
            "zipWithMatched",
            "mapMaybeMissing",
            "dropMissing",
            "filterMissing",
            "preserveMissing",
            "mapMissing",
            "traverseMissing",
            "filterAMissing",
            "mapMaybeMissing",
            "WhenMissing",
            "WhenMatched",
            "SimpleWhenMissing",
            "SimpleWhenMatched",
            "runWhenMissing",
            "runWhenMatched",
            "zipWithAMatched",
            "zipWithMaybeMatched",
            "mergeA",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for type_name_chirho in &[
            "WhenMissing",
            "WhenMatched",
            "SimpleWhenMissing",
            "SimpleWhenMatched",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Merge.Lazy".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Merge.Strict".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (already exists but add Data.IORef.Strict variant)
    // Data.Map.Internal — same exports as Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Map",
            "empty",
            "singleton",
            "insert",
            "lookup",
            "member",
            "delete",
            "fromList",
            "toList",
            "toAscList",
            "toDescList",
            "size",
            "null",
            "keys",
            "elems",
            "union",
            "unionWith",
            "unions",
            "unionsWith",
            "intersection",
            "intersectionWith",
            "difference",
            "map",
            "mapWithKey",
            "filter",
            "filterWithKey",
            "foldlWithKey",
            "foldrWithKey",
            "foldlWithKey'",
            "insertWith",
            "insertWithKey",
            "adjust",
            "alter",
            "findWithDefault",
            "mapKeys",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntMap",
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
            "foldlWithKey",
            "foldrWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Data.Map / Data.Map.Strict / Data.Map.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "notMember",
            "findWithDefault",
            "union",
            "unionWith",
            "unions",
            "unionsWith",
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
            "fromListWith",
            "toAscList",
            "toDescList",
            "elems",
            "keys",
            "size",
            "null",
            "isSubmapOf",
            "isSubmapOfBy",
            "isProperSubmapOf",
            "isProperSubmapOfBy",
            "mapKeys",
            "mapKeysMonotonic",
            "adjust",
            "alter",
            "insertWith",
            "insertWithKey",
            "intersectionWith",
            "intersectionWithKey",
            "differenceWith",
            "differenceWithKey",
            "mergeWithKey",
            "updateWithKey",
            "foldlWithKey",
            "foldrWithKey'",
            "traverseWithKey",
            "mapAccum",
            "mapAccumWithKey",
            "split",
            "splitLookup",
            "lookupMin",
            "lookupMax",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "mapMaybe",
            "mapMaybeWithKey",
            "mapEither",
            "mapEitherWithKey",
            "partition",
            "partitionWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.Map", "Data.Map.Strict", "Data.Map.Lazy"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.IntMap
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
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
            "toAscList",
            "keys",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap / Data.IntMap.Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["IntMap", "Key"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "lookup",
            "member",
            "notMember",
            "findWithDefault",
            "union",
            "unionWith",
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
            "toAscList",
            "elems",
            "keys",
            "size",
            "null",
            "mapKeys",
            "mapKeysMonotonic",
            "adjust",
            "alter",
            "intersectionWith",
            "differenceWith",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.IntMap", "Data.IntMap.Strict", "Data.IntMap.Lazy"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Map — extend with missing commonly-used exports
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mapKeys",
            "mapKeysMonotonic",
            "mapKeysWith",
            "adjust",
            "alter",
            "filter",
            "unions",
            "unionsWith",
            "intersectionWith",
            "differenceWith",
            "isSubmapOf",
            "isSubmapOfBy",
            "isProperSubmapOf",
            "isProperSubmapOfBy",
            "restrictKeys",
            "withoutKeys",
            "(!?)",
            "(!)",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "lookupMin",
            "lookupMax",
            "updateWithKey",
            "alterF",
            "updateMin",
            "updateMax",
            "insertWithKey",
            "foldlWithKey",
            "foldrWithKey'",
            "mapAccum",
            "mapAccumWithKey",
            "mergeWithKey",
            "foldMapWithKey",
            "traverseWithKey",
            "keysSet",
            "valid",
            "fromSet",
            "mapWithKey",
            "filterWithKey",
            "partitionWithKey",
            "fromListWith",
            "fromAscListWith",
            "mapKeysWith",
            "fromDistinctAscList",
            "toAscList",
            "toDescList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.Map", "Data.Map.Strict", "Data.Map.Lazy"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Map.Internal.Debug
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["showTree", "showTreeWith", "valid"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Map.Internal.Debug".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap.Internal.Debug
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["showTree", "showTreeWith", "valid"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap.Internal.Debug".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap / Data.IntMap.Strict / Data.IntMap.Lazy / Data.IntMap.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntMap",
            "empty",
            "singleton",
            "fromList",
            "fromListWith",
            "fromAscList",
            "fromAscListWith",
            "fromDistinctAscList",
            "toList",
            "toAscList",
            "toDescList",
            "insert",
            "insertWith",
            "insertWithKey",
            "delete",
            "adjust",
            "adjustWithKey",
            "update",
            "updateWithKey",
            "alterF",
            "updateMin",
            "updateMax",
            "alter",
            "lookup",
            "findWithDefault",
            "member",
            "notMember",
            "null",
            "size",
            "union",
            "unionWith",
            "unionWithKey",
            "unions",
            "unionsWith",
            "difference",
            "differenceWith",
            "intersection",
            "intersectionWith",
            "map",
            "mapWithKey",
            "mapAccum",
            "mapAccumWithKey",
            "mapKeys",
            "mapKeysWith",
            "fold",
            "foldWithKey",
            "foldr",
            "foldl",
            "foldr'",
            "foldl'",
            "foldMapWithKey",
            "foldrWithKey",
            "foldlWithKey",
            "foldrWithKey'",
            "foldlWithKey'",
            "traverseWithKey",
            "elems",
            "keys",
            "assocs",
            "keysSet",
            "filter",
            "filterWithKey",
            "partition",
            "partitionWithKey",
            "mapMaybe",
            "mapMaybeWithKey",
            "mapEither",
            "mapEitherWithKey",
            "split",
            "splitLookup",
            "isSubmapOf",
            "isSubmapOfBy",
            "isProperSubmapOf",
            "isProperSubmapOfBy",
            "mergeWithKey",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "minViewWithKey",
            "maxViewWithKey",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &[
            "Data.IntMap",
            "Data.IntMap.Strict",
            "Data.IntMap.Lazy",
            "Data.IntMap.Internal",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }
}
