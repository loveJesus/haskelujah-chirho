// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: sets, sequences, trees and graphs. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for sets, sequences, trees and graphs.
pub(super) fn push_collections_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "setEmpty",
            "setInsert",
            "setMember",
            "setDelete",
            "setFromList",
            "setToList",
            "setSize",
            "setUnion",
            "setIntersection",
            "setDifference",
            "setNull",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &["SetEmpty", "SetNode"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["SetEmpty", "SetNode"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // Data.Set.Internal — same exports as Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Set",
            "empty",
            "singleton",
            "insert",
            "member",
            "delete",
            "fromList",
            "toList",
            "toAscList",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "map",
            "filter",
            "foldl'",
            "foldr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set.Internal".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence (Data.Seq)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Seq",
            "empty",
            "singleton",
            "fromList",
            "length",
            "null",
            "index",
            "adjust",
            "update",
            "take",
            "drop",
            "splitAt",
            "filter",
            "sort",
            "zip",
            "zipWith",
            "ViewL",
            "viewl",
            "ViewR",
            "viewr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Seq", &["Empty", ":<|"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.IntSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntSet",
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
            "difference",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntSet".to_string(),
            exports_chirho,
        });
    }

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "notMember",
            "size",
            "null",
            "union",
            "intersection",
            "difference",
            "\\\\",
            "isSubsetOf",
            "isProperSubsetOf",
            "disjoint",
            "map",
            "mapMonotonic",
            "filter",
            "foldl'",
            "foldr",
            "toList",
            "fromList",
            "toAscList",
            "toDescList",
            "elems",
            "powerSet",
            "lookupMin",
            "lookupMax",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "unions",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence (Data.Sequence)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Seq", &["Empty", ":<|", ":|>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ViewL", &["EmptyL", ":<"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ViewR", &["EmptyR", ":>"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "(<|)",
            "(|>)",
            "(><)",
            "fromList",
            "length",
            "null",
            "index",
            "adjust",
            "update",
            "take",
            "drop",
            "splitAt",
            "viewl",
            "viewr",
            "filter",
            "sort",
            "reverse",
            "zip",
            "zipWith",
            "unzip",
            "replicate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.IntSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "notMember",
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
            "toAscList",
            "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntSet".to_string(),
            exports_chirho,
        });
    }

    // Data.Tree
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "rootLabel",
            "subForest",
            "drawTree",
            "drawForest",
            "flatten",
            "levels",
            "foldTree",
            "unfoldTree",
            "unfoldForest",
            "unfoldTreeM",
            "unfoldForestM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Tree";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Node"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Forest";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tree".to_string(),
            exports_chirho,
        });
    }

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "empty",
            "singleton",
            "insert",
            "delete",
            "member",
            "notMember",
            "null",
            "size",
            "union",
            "intersection",
            "difference",
            "isSubsetOf",
            "toList",
            "fromList",
            "toAscList",
            "toDescList",
            "map",
            "filter",
            "fromDistinctAscList",
            "foldl'",
            "foldr",
            "elems",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "lookupMin",
            "lookupMax",
            "mapMonotonic",
            "powerSet",
            "cartesianProduct",
            "isSubsetOf",
            "isProperSubsetOf",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Set";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"Seq";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "empty",
            "singleton",
            "fromList",
            "toList",
            "length",
            "null",
            "index",
            "lookup",
            "(<|)",
            "(|>)",
            "(><)",
            "filter",
            "sort",
            "reverse",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.Sequence / Data.Sequence.Internal (extend)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Seq",
            "empty",
            "singleton",
            "fromList",
            "toList",
            "length",
            "null",
            "index",
            "adjust",
            "update",
            "take",
            "drop",
            "splitAt",
            "viewl",
            "viewr",
            "ViewL",
            "ViewR",
            "EmptyL",
            "EmptyR",
            ":<",
            ":>",
            "|>",
            "<|",
            "><",
            "filter",
            "sort",
            "reverse",
            "zip",
            "zipWith",
            "unzip",
            "lookup",
            "insertAt",
            "deleteAt",
            "findIndexL",
            "findIndexR",
            "foldlWithIndex",
            "foldrWithIndex",
            "foldMapWithIndex",
            "mapWithIndex",
            "traverseWithIndex",
            "replicate",
            "replicateA",
            "replicateM",
            "unfoldl",
            "unfoldr",
            "iterateN",
            "|>",
            "<|",
            "><",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Seq", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["ViewL", "ViewR"] {
            let (k_chirho, v_chirho) =
                mk_type_chirho(name_chirho, &["EmptyL", "EmptyR", ":<", ":>"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &["Data.Sequence", "Data.Sequence.Internal"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.IntSet / Data.IntSet.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntSet",
            "empty",
            "singleton",
            "fromList",
            "fromAscList",
            "fromDistinctAscList",
            "toList",
            "toAscList",
            "toDescList",
            "insert",
            "delete",
            "member",
            "notMember",
            "null",
            "size",
            "union",
            "unions",
            "difference",
            "intersection",
            "isSubsetOf",
            "isProperSubsetOf",
            "filter",
            "partition",
            "split",
            "splitMember",
            "findMin",
            "findMax",
            "deleteMin",
            "deleteMax",
            "elems",
            "fold",
            "foldl",
            "foldr",
            "foldl'",
            "foldr'",
            "map",
            "disjoint",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Data.IntSet", "Data.IntSet.Internal"] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Graph
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Graph",
            "Table",
            "Bounds",
            "Edge",
            "Vertex",
            "Forest",
            "Tree",
            "graphFromEdges",
            "graphFromEdges'",
            "buildG",
            "transposeG",
            "vertices",
            "edges",
            "outdegree",
            "indegree",
            "dff",
            "dfs",
            "topSort",
            "components",
            "scc",
            "bcc",
            "reachable",
            "path",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Graph".to_string(),
            exports_chirho,
        });
    }

    // Data.Tree
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Tree",
            "Node",
            "rootLabel",
            "subForest",
            "Forest",
            "drawTree",
            "drawForest",
            "flatten",
            "levels",
            "foldTree",
            "unfoldTree",
            "unfoldForest",
            "unfoldTreeM",
            "unfoldForestM",
            "unfoldTreeM_BF",
            "unfoldForestM_BF",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Tree", &["Node"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tree".to_string(),
            exports_chirho,
        });
    }
}
