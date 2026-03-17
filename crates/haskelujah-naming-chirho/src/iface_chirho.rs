// For God so loved the world that he gave his only begotten Son, that whoever
// believes in him should not perish but have eternal life. — John 3:16

//! # Module interfaces
//!
//! A `ModuleIfaceChirho` represents the public API of a compiled module — the
//! names (values, types, constructors) that other modules may import from it.
//! Building an interface applies the module's export list to its declarations.

use std::collections::HashMap;

use haskelujah_ast_chirho::decl_chirho::{ConDeclChirho, DeclChirho};
use haskelujah_ast_chirho::module_chirho::{ExportMembersChirho, ExportSpecChirho, ModuleChirho};
use haskelujah_span_chirho::SpanChirho;

// ---------------------------------------------------------------------------
// Interface types
// ---------------------------------------------------------------------------

/// An exported value (function, variable, data constructor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfaceValueChirho {
    pub name_chirho: String,
    pub span_chirho: SpanChirho,
}

/// An exported type with its available constructors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfaceTypeChirho {
    pub name_chirho: String,
    /// Constructor names that are exported alongside this type.
    pub constructors_chirho: Vec<String>,
    /// Class method names that are exported alongside this class.
    pub methods_chirho: Vec<String>,
    pub span_chirho: SpanChirho,
}

/// Everything a module exports.
#[derive(Debug, Clone, Default)]
pub struct IfaceExportsChirho {
    /// Value-level exports: functions, variables, data constructors.
    pub values_chirho: HashMap<String, IfaceValueChirho>,
    /// Type-level exports: type constructors, type classes.
    pub types_chirho: HashMap<String, IfaceTypeChirho>,
}

/// A compiled module's public interface.
#[derive(Debug, Clone)]
pub struct ModuleIfaceChirho {
    /// Fully qualified module name.
    pub name_chirho: String,
    /// The exported names.
    pub exports_chirho: IfaceExportsChirho,
}

// ---------------------------------------------------------------------------
// Building an interface from a module
// ---------------------------------------------------------------------------

/// Build a module interface from a parsed module. Applies the export list
/// to determine which names are publicly visible.
pub fn build_iface_chirho(module_chirho: &ModuleChirho) -> ModuleIfaceChirho {
    build_iface_with_imports_chirho(module_chirho, &[])
}

/// Build a module interface from a parsed module, with access to imported
/// module interfaces for handling `module Foo` re-exports in the export list.
pub fn build_iface_with_imports_chirho(
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
) -> ModuleIfaceChirho {
    let module_name_chirho = module_chirho.name_chirho.text_chirho().to_string();

    // First, collect ALL definitions in the module.
    let all_exports_chirho = collect_all_definitions_chirho(module_chirho);

    // Then filter by the export list.
    let exports_chirho = match &module_chirho.exports_chirho {
        None => {
            // No export list means export everything.
            all_exports_chirho
        }
        Some(specs_chirho) => filter_exports_chirho(
            &all_exports_chirho,
            specs_chirho,
            module_chirho,
            imported_ifaces_chirho,
        ),
    };

    ModuleIfaceChirho {
        name_chirho: module_name_chirho,
        exports_chirho,
    }
}

/// Build synthetic module interfaces for built-in modules like `Data.Map`,
/// `Data.Set`, `Data.List`, `Data.Char`, `Data.Maybe`, `Data.Either`.
/// These are generated at compile time and don't correspond to source files.
pub fn builtin_module_ifaces_chirho() -> Vec<ModuleIfaceChirho> {
    let mut modules_chirho = Vec::new();

    // Helper: create an IfaceValueChirho with DUMMY span
    let mk_val_chirho = |name_chirho: &str| -> (String, IfaceValueChirho) {
        (name_chirho.to_string(), IfaceValueChirho {
            name_chirho: name_chirho.to_string(),
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })
    };
    let mk_type_chirho = |name_chirho: &str, cons_chirho: &[&str]| -> (String, IfaceTypeChirho) {
        (name_chirho.to_string(), IfaceTypeChirho {
            name_chirho: name_chirho.to_string(),
            constructors_chirho: cons_chirho.iter().map(|c_chirho| c_chirho.to_string()).collect(),
            methods_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        })
    };

    // Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mapEmpty", "mapInsert", "mapLookup", "mapMember", "mapDelete",
            "mapFromList", "mapToList", "mapSize", "mapKeys", "mapElems",
            "mapFoldlWithKey", "mapInsertWith", "mapFindWithDefault", "mapAdjust",
            "mapUnionWith", "mapMap", "mapFilter", "mapNull",
            "mapInsertStr", "mapLookupStr", "mapMemberStr", "mapDeleteStr",
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

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "setEmpty", "setInsert", "setMember", "setDelete",
            "setFromList", "setToList", "setSize", "setUnion",
            "setIntersection", "setDifference", "setNull",
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

    // Data.List
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map", "filter", "foldr", "foldl", "head", "tail", "last", "init",
            "null", "length", "reverse", "zip", "zipWith", "unzip",
            "take", "drop", "takeWhile", "dropWhile", "span", "break",
            "elem", "notElem", "lookup", "sum", "product", "minimum", "maximum",
            "sort", "insert", "nub", "concat", "concatMap", "any", "all",
            "iterate", "scanl", "partition",
            "sortBy", "insertBy", "nubBy", "maximumBy", "minimumBy",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.List".to_string(),
            exports_chirho,
        });
    }

    // Data.Char
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ord", "chr", "isDigit", "isAlpha", "isAlphaNum", "isUpper", "isLower",
            "isSpace", "isLetter", "isPrint", "isControl", "isPunctuation", "isSeparator",
            "isAscii", "isLatin1", "isAsciiUpper", "isAsciiLower", "isHexDigit", "isOctDigit",
            "toLower", "toUpper", "toTitle", "digitToInt", "intToDigit",
            "showLitChar", "readLitChar", "lexLitChar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Char".to_string(),
            exports_chirho,
        });
    }

    // Data.Maybe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "maybe", "isJust", "isNothing", "fromMaybe", "fromJust",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Maybe", &["Nothing", "Just"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["Nothing", "Just"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Maybe".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newIORef", "readIORef", "writeIORef", "modifyIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.STM
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newTVar", "readTVar", "writeTVar",
            "newTVarIO", "readTVarIO",
            "atomically", "retry", "orElse",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent.STM".to_string(),
            exports_chirho,
        });
    }

    // Prelude — the implicit import every Haskell module gets
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        // Core types
        for (type_name_chirho, cons_chirho) in &[
            ("Bool", &["False", "True"][..]),
            ("Maybe", &["Nothing", "Just"][..]),
            ("Either", &["Left", "Right"][..]),
            ("Ordering", &["LT", "EQ", "GT"][..]),
            ("()", &["()"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_name_chirho, cons_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            for con_chirho in *cons_chirho {
                let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
                exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            }
        }
        // Prelude types without data constructors
        for name_chirho in &[
            "IO", "String", "Char", "Int", "Integer", "Float", "Double",
            "Rational", "ShowS", "ReadS", "FilePath", "IOError",
            "Num", "Eq", "Ord", "Show", "Read", "Enum", "Bounded",
            "Functor", "Applicative", "Monad", "MonadFail", "Semigroup", "Monoid",
            "Foldable", "Traversable",
            "Coercible",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Standard Prelude functions
        for name_chirho in &[
            // Numeric
            "abs", "signum", "negate", "fromInteger", "fromIntegral", "toInteger",
            "even", "odd", "max", "min", "div", "mod", "quot", "rem",
            "succ", "pred", "toEnum", "fromEnum", "minBound", "maxBound",
            "floor", "ceiling", "round", "truncate",
            // Boolean
            "not", "otherwise",
            // Tuple
            "fst", "snd", "curry", "uncurry",
            // Function
            "id", "const", "flip", "error", "undefined",
            // List
            "map", "filter", "foldr", "foldl", "head", "tail", "last", "init",
            "null", "length", "reverse", "concat", "concatMap",
            "take", "drop", "takeWhile", "dropWhile", "span",
            "zip", "zipWith", "unzip", "elem", "notElem", "lookup",
            "sum", "product", "minimum", "maximum", "any", "all",
            "iterate", "scanl", "words", "unwords", "lines", "unlines",
            // IO
            "putStrLn", "putStr", "putChar", "print",
            "getLine", "getChar", "getContents", "interact",
            "readFile", "writeFile", "appendFile",
            // Show/Read
            "show", "read",
            // Monad/Functor/Foldable/Traversable
            "fmap", "return", "mapM_", "sequence_", "when", "unless",
            "foldMap", "traverse",
            // Conversion
            "fromString", "fromList", "toList",
            // Comparison
            "compare",
            // String ops
            "intercalate",
            // Maybe/Either
            "maybe", "either", "fromMaybe", "isJust", "isNothing",
            // Data structures
            "sort",
            // Monad transformers
            "StateT", "runStateT", "evalStateT", "execStateT",
            "runState", "evalState", "execState",
            "get", "put", "modify", "bindStateT", "returnStateT",
            "MaybeT", "runMaybeT", "returnMaybeT", "bindMaybeT",
            // ReaderT
            "ReaderT", "runReaderT", "runReader", "ask", "local",
            "bindReaderT", "returnReaderT",
            // ExceptT
            "ExceptT", "runExceptT", "throwE", "returnExceptT",
            "bindExceptT", "catchE",
            // WriterT
            "WriterT", "runWriterT", "runWriter", "tell",
            "returnWriterT", "bindWriterT", "execWriterT", "execWriter",
            // NFData / deepseq
            "deepseq", "force", "evaluate", "$!!",
            // STM
            "newTVar", "readTVar", "writeTVar", "newTVarIO", "readTVarIO",
            "atomically", "retry", "orElse",
            // Data.Coerce
            "coerce",
            // Data.Function
            "on", "fix",
            // Numeric
            "realToFrac",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude".to_string(),
            exports_chirho,
        });
    }

    // Data.Kind — Type, Constraint
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Type", "Constraint"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Kind".to_string(),
            exports_chirho,
        });
    }

    // GHC.Exts — primops, coerce, IsList, etc.
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "coerce", "inline", "lazy", "oneShot", "noinline",
            "fromList", "fromListN", "toList", "groupWith", "sortWith",
            "the", "build", "augment", "reallyUnsafePtrEquality#",
            "withDict", "foldl'", "isTrue#", "proxy#",
            "unsafeCoerce#", "magicDict",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "IsList", "Item", "Constraint", "Type", "RuntimeRep",
            "Int#", "Word#", "Float#", "Double#", "Char#",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exts".to_string(),
            exports_chirho,
        });
    }

    // GHC.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Type", "Constraint", "RuntimeRep", "Levity", "Multiplicity", "Symbol"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Types".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal", "symbolVal", "sameNat", "sameSymbol",
            "someNatVal", "someSymbolVal",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Nat", "Symbol", "KnownNat", "KnownSymbol", "SomeNat", "SomeSymbol", "TypeError", "ErrorMessage"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["from", "to", "from1", "to1", "datatypeName", "moduleName", "packageName"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Generic", "Generic1", "Rep", "Rep1",
            "V1", "U1", "K1", "M1", "Rec0", "Par1", "Rec1",
            "D1", "C1", "S1", "D", "C", "S", "R",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Generics".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "when", "unless", "guard", "void", "join",
            "forM", "forM_", "mapM", "mapM_", "sequence", "sequence_",
            "forever", "foldM", "foldM_", "filterM",
            "mplus", "mzero", "msum", "ap", "liftM", "liftM2",
            "return", ">>=", ">>", "fail",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Monad", "MonadPlus", "MonadFail"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Fix
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("mfix");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadFix", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Fix".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "pure", "<*>", "*>", "<*", "liftA", "liftA2", "liftA3",
            "empty", "<|>", "some", "many", "optional",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Applicative", "Alternative"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Control.Arrow
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "arr", "first", "second", "***", "&&&",
            "returnA", "<<<", ">>>",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Arrow", "ArrowChoice", "ArrowApply", "ArrowLoop"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Arrow".to_string(),
            exports_chirho,
        });
    }

    // Data.Proxy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Proxy", &["Proxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Proxy");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("KProxy", &["KProxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("KProxy");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Proxy".to_string(),
            exports_chirho,
        });
    }

    // Data.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("coerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Coercible", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Coerce".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Equality
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["castWith", "gcastWith", "testEquality", "sym", "trans", "inner", "outer"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["(:~:)", "(:~~:)", "TestEquality"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Refl"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_val_chirho("Refl");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Equality".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["typeOf", "typeRep", "cast", "eqT", "gcast", "gcast1", "gcast2"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Typeable", "TypeRep", "Proxy"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Identity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("runIdentity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Type.Reflection
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["typeOf", "typeRep", "typeRepTyCon", "someTypeRep"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Typeable", "TypeRep", "SomeTypeRep", "TyCon"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Type.Reflection".to_string(),
            exports_chirho,
        });
    }

    // System.IO
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "putStr", "putStrLn", "print", "getLine", "getContents",
            "readFile", "writeFile", "appendFile",
            "hSetBuffering", "hGetContents", "hPutStr", "hPutStrLn",
            "hFlush", "hClose", "hSetEncoding",
            "stdin", "stdout", "stderr",
            "withFile", "openFile",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["IO", "Handle", "IOMode", "BufferMode"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newSTRef", "readSTRef", "writeSTRef", "modifySTRef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
    }

    // Data.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int", "Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Int".to_string(),
            exports_chirho,
        });
    }

    // Data.List.NonEmpty
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["head", "tail", "toList", "fromList", "map", "nonEmpty"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("NonEmpty", &["(:|)"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("(:|)");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.List.NonEmpty".to_string(),
            exports_chirho,
        });
    }

    // Data.Tuple.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Solo", "fst", "snd", "swap", "curry", "uncurry"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Solo", "Tuple0", "Tuple1", "Tuple2", "Tuple3", "Unit"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tuple.Experimental".to_string(),
            exports_chirho,
        });
    }

    // Data.Sum.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Sum2", "Sum3", "Sum4", "Sum5"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sum.Experimental".to_string(),
            exports_chirho,
        });
    }

    // GHC.StaticPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["staticKey", "deRefStaticPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StaticPtr", "IsStatic"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.StaticPtr".to_string(),
            exports_chirho,
        });
    }

    // Unsafe.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("unsafeCoerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("UnsafeEquality", &["UnsafeRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("UnsafeRefl");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Unsafe.Coerce".to_string(),
            exports_chirho,
        });
    }

    // Data.Void
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Void", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("absurd");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("vacuous");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Void".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mempty", "mappend", "mconcat", "getSum", "getProduct",
                             "getFirst", "getLast", "getAny", "getAll",
                             "getDual", "getEndo", "appEndo"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Monoid", "Sum", "Product", "First", "Last",
                             "Any", "All", "Dual", "Endo", "Ap", "Alt"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["(<>)", "sconcat", "stimes"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Semigroup", "Min", "Max", "First", "Last",
                             "WrappedMonoid", "Option", "Arg"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Semigroup".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["natVal", "natVal'", "someNatVal", "sameNat",
                             "cmpNat", "withSomeSNat", "withKnownNat"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Natural", "Nat", "KnownNat", "SomeNat", "SNat",
                             "CmpNat", "Div", "Mod", "Log2"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["toConstr", "gunfold", "gfoldl", "dataTypeOf",
                             "gmapT", "gmapQ", "gmapQl", "gmapQr", "gmapQi", "gmapM",
                             "mkConstr", "mkDataType", "constrType", "showConstr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Data", "Typeable", "DataType", "Constr", "ConstrRep", "DataRep",
                             "ConIndex", "Fixity"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.IO.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("liftIO");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadIO", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.IO.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Category
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["id", ".", "<<<", ">>>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Category", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Category".to_string(),
            exports_chirho,
        });
    }

    // Data.Ord
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["compare", "comparing", "clamp", "Down"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ord", "Ordering", "Down"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["LT", "EQ", "GT"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ord".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("lift");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("MonadTrans", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Class".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentityT", "mapIdentityT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IdentityT", &["IdentityT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Identity".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.State / .Lazy / .Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runStateT", "evalStateT", "execStateT",
                             "runState", "evalState", "execState",
                             "get", "put", "modify", "gets", "state"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["StateT", "State"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Control.Monad.Trans.State",
            "Control.Monad.Trans.State.Lazy",
            "Control.Monad.Trans.State.Strict",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Reader
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runReaderT", "mapReaderT", "withReaderT", "reader"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ReaderT", &["ReaderT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Reader".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Writer / .Lazy / .Strict
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runWriterT", "execWriterT", "mapWriterT", "tell", "listen", "pass", "writer"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("WriterT", &["WriterT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &[
            "Control.Monad.Trans.Writer",
            "Control.Monad.Trans.Writer.Lazy",
            "Control.Monad.Trans.Writer.Strict",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.Monad.Trans.Except
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runExceptT", "throwE", "catchE", "mapExceptT", "withExceptT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("ExceptT", &["ExceptT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Except".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Maybe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runMaybeT", "mapMaybeT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MaybeT", &["MaybeT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Maybe".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Reader
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["ask", "asks", "local", "reader",
                             "runReader", "runReaderT", "mapReaderT", "withReaderT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Reader", "ReaderT", "MonadReader"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Reader".to_string(),
            exports_chirho,
        });
    }

    // Control.Exception
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["throw", "throwIO", "catch", "handle", "try", "evaluate",
                             "bracket", "bracket_", "finally", "onException",
                             "throwTo", "mask", "mask_", "uninterruptibleMask",
                             "assert", "mapException", "displayException",
                             "toException", "fromException", "catches", "Handler"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Exception", "SomeException", "IOException", "ErrorCall",
                             "ArithException", "ArrayException", "AsyncException",
                             "NonTermination", "BlockedIndefinitelyOnMVar",
                             "BlockedIndefinitelyOnSTM", "Deadlock"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Exception".to_string(),
            exports_chirho,
        });
    }

    // Data.Function
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["id", "const", "flip", "fix", "on", "&"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Function".to_string(),
            exports_chirho,
        });
    }

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

    // GHC.Tuple
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Solo", "Unit", "Tuple0", "Tuple1", "Tuple2", "Tuple3",
                             "Tuple4", "Tuple5", "Tuple6", "Tuple7"] {
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

    // GHC.List
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["map", "filter", "head", "tail", "last", "init",
                             "null", "length", "foldr", "foldl", "foldl'",
                             "scanl", "scanl'", "scanr", "iterate", "iterate'",
                             "repeat", "replicate", "cycle",
                             "take", "drop", "splitAt", "takeWhile", "dropWhile",
                             "span", "break", "reverse", "and", "or", "any", "all",
                             "elem", "notElem", "lookup", "zip", "zip3", "zipWith",
                             "unzip", "unzip3", "concat", "concatMap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.List".to_string(),
            exports_chirho,
        });
    }

    // GHC.Base
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["id", "const", "flip", ".", "$", "otherwise",
                             "map", "foldr", "build", "augment",
                             "fmap", "<$>", "pure", "<*>", "return", ">>=", ">>",
                             "eqString", "bindIO", "returnIO", "thenIO",
                             "seq", "maxInt", "minInt"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Functor", "Applicative", "Monad", "Semigroup", "Monoid",
                             "String", "Opaque", "SPEC"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Base".to_string(),
            exports_chirho,
        });
    }

    // GHC.Classes
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["==", "/=", "<", "<=", ">", ">=", "compare", "max", "min",
                             "not", "&&", "||", "divInt#", "modInt#"] {
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

    // GHC.Num
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["+", "-", "*", "negate", "abs", "signum", "fromInteger",
                             "subtract", "integerToInt", "naturalToWord"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Num", "Integer", "Natural"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num".to_string(),
            exports_chirho,
        });
    }

    // GHC.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["show", "showsPrec", "showString", "showChar", "showParen",
                             "shows", "showList__"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Show", "ShowS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Show".to_string(),
            exports_chirho,
        });
    }

    // GHC.Read
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["read", "reads", "readParen", "lex", "readsPrec", "readList"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Read", "ReadS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Read".to_string(),
            exports_chirho,
        });
    }

    // GHC.Enum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["succ", "pred", "toEnum", "fromEnum",
                             "enumFrom", "enumFromThen", "enumFromTo", "enumFromThenTo",
                             "minBound", "maxBound"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Enum", "Bounded"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Enum".to_string(),
            exports_chirho,
        });
    }

    // GHC.Real
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["div", "mod", "quot", "rem", "divMod", "quotRem",
                             "toInteger", "toRational", "fromIntegral", "realToFrac",
                             "%", "numerator", "denominator", "reduce"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Integral", "Real", "RealFrac", "Fractional", "Ratio", "Rational"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Real".to_string(),
            exports_chirho,
        });
    }

    // GHC.Float
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["pi", "exp", "log", "sqrt", "sin", "cos", "tan",
                             "asin", "acos", "atan", "sinh", "cosh", "tanh",
                             "float2Double", "double2Float",
                             "isNaN", "isInfinite", "isDenormalized",
                             "isNegativeZero", "isIEEE",
                             "integerToFloat#", "integerToDouble#",
                             "rationalToFloat", "rationalToDouble"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Float", "Double", "Floating", "RealFloat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Float".to_string(),
            exports_chirho,
        });
    }

    // GHC.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Int".to_string(),
            exports_chirho,
        });
    }

    // GHC.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Word".to_string(),
            exports_chirho,
        });
    }

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
        for name_chirho in &["unsafePerformIO", "unsafeInterleaveIO", "unsafeDupablePerformIO",
                             "stToIO", "ioToST", "throwIO", "catchException"] {
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
        for name_chirho in &["newIORef", "readIORef", "writeIORef", "modifyIORef", "atomicModifyIORef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IORef".to_string(),
            exports_chirho,
        });
    }

    // GHC.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newMVar", "newEmptyMVar", "takeMVar", "putMVar",
                             "readMVar", "tryTakeMVar", "tryPutMVar", "isEmptyMVar"] {
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

    // Control.Monad.State / Control.Monad.State.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["get", "put", "modify", "gets", "state",
                             "runState", "evalState", "execState",
                             "runStateT", "evalStateT", "execStateT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["State", "StateT", "MonadState"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State.Class".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.State.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Writer / Control.Monad.Writer.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["tell", "listen", "pass", "writer",
                             "runWriter", "execWriter",
                             "runWriterT", "execWriterT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Writer", "WriterT", "MonadWriter"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer.Class".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer.Strict".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Writer.Lazy".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Except
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["throwError", "catchError", "runExcept", "runExceptT",
                             "mapExcept", "mapExceptT", "withExcept", "withExceptT"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Except", "ExceptT", "MonadError"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Except".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Identity
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Identity");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Identity".to_string(),
            exports_chirho,
        });
    }

    // Data.Ratio
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["%", "numerator", "denominator", "approxRational"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ratio", "Rational"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ratio".to_string(),
            exports_chirho,
        });
    }

    // Data.Complex
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["realPart", "imagPart", "mkPolar", "cis",
                             "polar", "magnitude", "phase", "conjugate"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Complex", &[":+"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho(":+");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Complex".to_string(),
            exports_chirho,
        });
    }

    // Data.Map.Strict (alias to Data.Map)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "empty", "singleton", "insert", "insertWith", "delete",
            "lookup", "member", "findWithDefault", "adjust", "update",
            "union", "unionWith", "intersection", "intersectionWith", "difference",
            "map", "mapWithKey", "filter", "filterWithKey",
            "foldr", "foldl", "foldrWithKey", "foldlWithKey",
            "null", "size", "keys", "elems", "toList", "fromList",
            "toAscList", "toDescList", "fromAscList",
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

    // Data.Set (expanded with standard API names)
    // (original Data.Set uses custom names; this covers standard containers API)

    // Data.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("fromString");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("IsString", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("String", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.String".to_string(),
            exports_chirho,
        });
    }

    // Data.Tuple
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["fst", "snd", "curry", "uncurry", "swap"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Solo"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["MkSolo"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k_chirho, v_chirho) = mk_val_chirho("MkSolo");
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tuple".to_string(),
            exports_chirho,
        });
    }

    // Data.Either
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["either", "lefts", "rights", "isLeft", "isRight",
                             "fromLeft", "fromRight", "partitionEithers"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Either", &["Left", "Right"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["Left", "Right"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Either".to_string(),
            exports_chirho,
        });
    }

    // Data.Bool
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bool", "not", "otherwise", "&&", "||"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bool", &["False", "True"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["False", "True"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bool".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["fmap", "<$>", "<$", "$>", "void", "<&>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["fold", "foldMap", "foldl", "foldr", "foldl'", "foldr'",
                             "toList", "null", "length", "elem", "maximum", "minimum",
                             "sum", "product", "any", "all", "and", "or",
                             "concat", "concatMap", "asum", "find",
                             "mapM_", "forM_", "sequence_", "for_", "traverse_"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Traversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["traverse", "sequenceA", "mapM", "sequence",
                             "for", "forM", "mapAccumL", "mapAccumR"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Traversable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Traversable".to_string(),
            exports_chirho,
        });
    }

    // Data.Bifunctor
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["bimap", "first", "second"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Bifunctor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bifunctor".to_string(),
            exports_chirho,
        });
    }

    // Prelude.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        // Re-export everything that Prelude exports (approximation)
        for name_chirho in &["id", "const", "flip", "error", "undefined",
                             "fmap", "pure", "return", "show", "print",
                             "putStrLn", "putStr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude.Experimental".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Zip
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mzip", "mzipWith", "munzip"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MonadZip", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Zip".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["callStack", "prettyCallStack", "getCallStack",
                             "currentCallStack", "withFrozenCallStack",
                             "freezeCallStack", "emptyCallStack", "pushCallStack"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["CallStack", "HasCallStack", "SrcLoc"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Stack".to_string(),
            exports_chirho,
        });
    }

    // GHC.Err
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["error", "errorWithoutStackTrace", "undefined"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Err".to_string(),
            exports_chirho,
        });
    }

    // GHC.Prim
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Int#", "Word#", "Float#", "Double#", "Char#", "Addr#",
            "MutableByteArray#", "ByteArray#", "Array#", "MutableArray#",
            "SmallArray#", "SmallMutableArray#",
            "MutVar#", "TVar#", "MVar#", "State#", "RealWorld",
            "Weak#", "StableName#", "StablePtr#",
            "Proxy#", "TYPE", "RuntimeRep", "LiftedRep", "UnliftedRep",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "seq", "realWorld#", "proxy#", "void#", "coerce",
            // Arithmetic primops
            "+#", "-#", "*#", "negateInt#", "quotInt#", "remInt#",
            "+##", "-##", "*##", "/##", "negateDouble#",
            "plusFloat#", "minusFloat#", "timesFloat#", "divideFloat#", "negateFloat#",
            "plusWord#", "minusWord#", "timesWord#", "timesWord2#", "quotWord#", "remWord#",
            // Comparison primops
            ">#", ">=#", "==#", "/=#", "<#", "<=#",
            "gtWord#", "geWord#", "eqWord#", "neWord#", "ltWord#", "leWord#",
            // Conversion primops
            "int2Word#", "word2Int#", "int2Double#", "double2Int#",
            "int2Float#", "float2Int#", "float2Double#", "double2Float#",
            "chr#", "ord#", "word2Double#", "word2Float#",
            // Array primops
            "newArray#", "readArray#", "writeArray#", "indexArray#",
            "sizeofArray#", "sizeofMutableArray#",
            "newByteArray#", "readIntArray#", "writeIntArray#", "indexIntArray#",
            "sizeofByteArray#", "sizeofMutableByteArray#",
            "unsafeFreezeArray#", "unsafeThawArray#",
            // MutVar primops
            "newMutVar#", "readMutVar#", "writeMutVar#",
            // MVar primops
            "newMVar#", "takeMVar#", "putMVar#", "tryTakeMVar#", "tryPutMVar#",
            // Weak pointer / stable name primops
            "mkWeak#", "deRefWeak#", "finalizeWeak#",
            "makeStableName#", "eqStableName#", "stableNameToInt#",
            // Misc primops
            "dataToTag#", "tagToEnum#",
            "reallyUnsafePtrEquality#",
            "touch#", "noDuplicate#",
            "raise#", "raiseIO#", "catch#",
            "maskAsyncExceptions#", "unmaskAsyncExceptions#",
            "atomically#", "retry#", "catchRetry#", "catchSTM#",
            "newTVar#", "readTVar#", "readTVarIO#", "writeTVar#",
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

    // Data.Char (already exists but GHC.Char doesn't)
    // GHC.Char
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["chr", "ord", "eqChar", "neChar"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Char", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Char".to_string(),
            exports_chirho,
        });
    }

    // System.IO.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["unsafePerformIO", "unsafeInterleaveIO", "unsafeDupablePerformIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (already exists via Data.IORef above, add GHC.IORef variant)

    // Control.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["forkIO", "forkOS", "killThread", "throwTo",
                             "threadDelay", "myThreadId", "yield"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ThreadId", "MVar"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newMVar", "newEmptyMVar", "takeMVar", "putMVar",
                             "readMVar", "swapMVar", "tryTakeMVar", "tryPutMVar",
                             "isEmptyMVar", "withMVar", "modifyMVar", "modifyMVar_"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
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
            name_chirho: "Control.DeepSeq".to_string(),
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

    // Data.Bits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[".&.", ".|.", "xor", "complement", "shift", "shiftL", "shiftR",
                             "rotate", "rotateL", "rotateR", "bit", "setBit", "clearBit",
                             "complementBit", "testBit", "bitSizeMaybe", "bitSize",
                             "isSigned", "popCount", "zeroBits", "finiteBitSize"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Bits", "FiniteBits"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bits".to_string(),
            exports_chirho,
        });
    }

    // Foreign / Foreign.Ptr / Foreign.C / Foreign.Storable / Foreign.ForeignPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Ptr", "FunPtr", "IntPtr", "WordPtr", "ForeignPtr",
                             "StablePtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["nullPtr", "plusPtr", "castPtr", "alignPtr",
                             "ptrToIntPtr", "intPtrToPtr",
                             "peek", "poke", "sizeOf", "alignment",
                             "malloc", "free", "alloca", "allocaBytes",
                             "newForeignPtr", "withForeignPtr", "mallocForeignPtr",
                             "newStablePtr", "deRefStablePtr", "freeStablePtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Storable"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Push as multiple module names
        for mod_name_chirho in &[
            "Foreign", "Foreign.Ptr", "Foreign.ForeignPtr", "Foreign.Storable",
            "Foreign.StablePtr", "Foreign.Marshal", "Foreign.Marshal.Alloc",
            "Foreign.Marshal.Utils", "Foreign.C", "Foreign.C.Types",
            "Foreign.C.String",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
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

    // Numeric
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["showInt", "showHex", "showOct", "showFloat",
                             "readInt", "readHex", "readOct", "readDec",
                             "showSigned", "readSigned", "floatToDigits",
                             "showIntAtBase", "readFloat"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric".to_string(),
            exports_chirho,
        });
    }

    // Text.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["show", "showsPrec", "showString", "showChar", "showParen", "shows"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Show", "ShowS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Show".to_string(),
            exports_chirho,
        });
    }

    // Text.Read
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["read", "reads", "readParen", "lex", "readPrec", "readListPrec"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Read", "ReadS", "ReadPrec"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Read".to_string(),
            exports_chirho,
        });
    }

    // GHC.Records
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("getField");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("setField");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["HasField"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Records".to_string(),
            exports_chirho,
        });
    }

    // GHC.OverloadedLabels
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("fromLabel");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("IsLabel", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.OverloadedLabels".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Bool
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["If", "Not", "type (&&)", "type (||)"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Bool".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Ord
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Compare", "OrderingI"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Ord".to_string(),
            exports_chirho,
        });
    }

    // GHC.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["naturalToInteger", "integerToNatural", "naturalToWord"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Natural".to_string(),
            exports_chirho,
        });
    }

    // Numeric.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Const");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getConst");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Classes
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["eq1", "compare1", "showsPrec1", "readsPrec1",
                             "liftEq", "liftCompare", "liftShowsPrec", "liftReadsPrec",
                             "eq2", "compare2", "showsPrec2", "readsPrec2",
                             "liftEq2", "liftCompare2", "liftShowsPrec2", "liftReadsPrec2"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Eq1", "Ord1", "Show1", "Read1", "Eq2", "Ord2", "Show2", "Read2"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Classes".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Compose");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getCompose");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Product
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Pair"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Pair");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Product".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Sum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["InL", "InR"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for con_chirho in &["InL", "InR"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Sum".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH — Template Haskell types and Q monad
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            // Q monad
            "Q", "runQ", "newName", "reify", "location",
            // Syntax construction
            "mkName", "nameBase", "nameModule",
            // Expression constructors
            "litE", "varE", "conE", "appE", "infixE", "lamE", "tupE", "listE",
            "sigE", "recConE", "recUpdE", "letE", "caseE", "doE",
            // Pattern constructors
            "litP", "varP", "conP", "tupP", "listP", "wildP", "asP",
            // Type constructors
            "conT", "varT", "appT", "arrowT", "listT", "tupleT", "sigT", "forallT",
            // Declaration constructors
            "funD", "valD", "dataD", "newtypeD", "tySynD", "classD", "instanceD",
            "sigD", "pragInlD",
            // Clause and body
            "clause", "normalB", "guardedB",
            // Literals
            "integerL", "rationalL", "charL", "stringL",
            // Lift class
            "lift", "liftTyped",
            // Misc
            "reportError", "reportWarning", "recover",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Exp", "Pat", "Type", "Dec", "Body", "Clause", "Lit", "Name",
            "Info", "Loc", "Range", "Guard", "Stmt", "Match", "Con",
            "Strict", "FunDep", "Pred", "TyVarBndr",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Syntax
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Lift", "lift", "liftTyped", "mkName", "nameBase",
            "Q", "runQ", "newName", "reify",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Name", "Exp", "Pat", "Type", "Dec", "Lit"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Syntax".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Lib
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "litE", "varE", "conE", "appE", "lamE", "tupE", "listE",
            "litP", "varP", "conP", "tupP", "wildP",
            "conT", "varT", "appT", "arrowT",
            "funD", "valD", "sigD", "clause", "normalB",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Language.Haskell.TH.Lib".to_string(),
            exports_chirho,
        });
    }

    // GHC.Conc — concurrent primitives
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "forkIO", "killThread", "threadDelay", "myThreadId",
            "STM", "atomically", "retry", "orElse",
            "TVar", "newTVar", "readTVar", "writeTVar",
            "newTVarIO", "readTVarIO",
            "throwSTM", "catchSTM",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STM", "TVar", "ThreadId"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Conc".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (already exists but add Data.IORef.Strict variant)
    // Data.Map.Internal — same exports as Data.Map
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Map", "empty", "singleton", "insert", "lookup", "member",
            "delete", "fromList", "toList", "toAscList", "toDescList",
            "size", "null", "keys", "elems", "union", "unionWith",
            "intersection", "intersectionWith", "difference",
            "map", "mapWithKey", "filter", "filterWithKey",
            "foldlWithKey", "foldrWithKey", "foldlWithKey'",
            "insertWith", "insertWithKey", "adjust", "alter",
            "findWithDefault", "mapKeys",
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

    // Data.Set.Internal — same exports as Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Set", "empty", "singleton", "insert", "member", "delete",
            "fromList", "toList", "toAscList", "size", "null",
            "union", "intersection", "difference",
            "map", "filter", "foldl'", "foldr",
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
            "Seq", "empty", "singleton", "fromList",
            "length", "null", "index", "adjust", "update",
            "take", "drop", "splitAt",
            "filter", "sort", "zip", "zipWith",
            "ViewL", "viewl", "ViewR", "viewr",
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

    // Data.IntMap
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntMap", "empty", "singleton", "insert", "lookup", "member",
            "delete", "fromList", "toList", "size", "null",
            "union", "unionWith", "intersection", "difference",
            "map", "filter", "foldlWithKey", "foldrWithKey",
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

    // Data.IntSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IntSet", "empty", "singleton", "insert", "member", "delete",
            "fromList", "toList", "size", "null",
            "union", "intersection", "difference",
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

    // Data.HashMap.Strict / Data.HashMap.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "HashMap", "empty", "singleton", "insert", "lookup", "member",
            "delete", "fromList", "toList", "size", "null",
            "union", "unionWith", "intersection", "difference",
            "map", "filter", "foldlWithKey'", "foldrWithKey",
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
            "HashSet", "empty", "singleton", "insert", "member", "delete",
            "fromList", "toList", "size", "null", "union", "intersection",
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

    // Data.Text
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Text", "pack", "unpack", "empty", "singleton",
            "cons", "snoc", "append", "head", "tail", "last", "init",
            "null", "length", "map", "intercalate", "intersperse",
            "transpose", "reverse", "toLower", "toUpper", "toTitle",
            "foldl", "foldl'", "foldr", "concat", "concatMap",
            "any", "all", "maximum", "minimum",
            "take", "drop", "splitAt", "takeWhile", "dropWhile",
            "strip", "stripStart", "stripEnd",
            "words", "unwords", "lines", "unlines",
            "isPrefixOf", "isSuffixOf", "isInfixOf",
            "replace", "breakOn", "splitOn",
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
            "Text", "pack", "unpack", "empty", "fromStrict", "toStrict",
            "null", "length", "map", "intercalate", "concat",
            "take", "drop", "splitAt",
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

    // Data.ByteString
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ByteString", "empty", "singleton", "pack", "unpack",
            "cons", "snoc", "append", "head", "tail", "last", "init",
            "null", "length", "map", "reverse", "intercalate",
            "foldl", "foldl'", "foldr", "concat", "concatMap",
            "take", "drop", "splitAt", "takeWhile", "dropWhile",
            "isPrefixOf", "isSuffixOf", "isInfixOf",
            "readFile", "writeFile",
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

    // Data.ByteString.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ByteString", "empty", "pack", "unpack", "fromStrict", "toStrict",
            "null", "length", "map", "concat", "take", "drop",
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
            exports_chirho,
        });
    }

    // Data.Vector
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Vector", "empty", "singleton", "fromList", "toList",
            "length", "null", "head", "tail", "last", "init",
            "map", "filter", "foldl", "foldl'", "foldr",
            "take", "drop", "slice", "zip", "zipWith",
            "generate", "replicate", "cons", "snoc",
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

    // GHC.ForeignPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ForeignPtr", "newForeignPtr", "newForeignPtr_",
            "withForeignPtr", "castForeignPtr", "mallocForeignPtr",
            "FinalizerPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ForeignPtr", "FinalizerPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ForeignPtr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr", "nullPtr", "castPtr", "plusPtr", "alignPtr",
            "FunPtr", "nullFunPtr", "castFunPtr",
            "WordPtr", "IntPtr", "ptrToWordPtr", "wordPtrToPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ptr", "FunPtr", "WordPtr", "IntPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Storable", "sizeOf", "alignment", "peek", "poke",
            "peekByteOff", "pokeByteOff", "peekElemOff", "pokeElemOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Marshal.Alloc
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "malloc", "mallocBytes", "calloc", "callocBytes",
            "realloc", "reallocBytes", "free", "alloca", "allocaBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Marshal.Alloc".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt", "CUInt", "CLong", "CULong", "CChar", "CUChar",
            "CShort", "CUShort", "CFloat", "CDouble", "CSize",
            "CLLong", "CULLong", "CBool", "CWchar",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[name_chirho]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k2_chirho, v2_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k2_chirho, v2_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Types".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CString", "CStringLen",
            "newCString", "newCStringLen", "withCString", "withCStringLen",
            "peekCString", "peekCStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.String".to_string(),
            exports_chirho,
        });
    }

    // Foreign (umbrella module)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr", "nullPtr", "castPtr", "plusPtr",
            "FunPtr", "nullFunPtr", "castFunPtr",
            "ForeignPtr", "newForeignPtr", "withForeignPtr",
            "Storable", "sizeOf", "alignment", "peek", "poke",
            "malloc", "free", "alloca",
            "CInt", "CUInt", "CLong", "CChar", "CDouble",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign".to_string(),
            exports_chirho,
        });
    }

    // Data.Char (extend existing with more functions)
    // Note: Data.Char already exists above, so this adds Data.Char.Internal
    // GHC.Unicode
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "isAlpha", "isAlphaNum", "isDigit", "isUpper", "isLower",
            "isSpace", "isPrint", "isControl", "isPunctuation",
            "toUpper", "toLower", "toTitle",
            "generalCategory", "GeneralCategory",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Unicode".to_string(),
            exports_chirho,
        });
    }

    // Data.Array
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Array", "array", "listArray", "accumArray",
            "bounds", "indices", "elems", "assocs",
            "ixmap", "amap",
            "Ix", "range", "index", "inRange", "rangeSize",
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

    // Data.IORef (already exists — add Data.IORef.Strict alias)
    // System.IO.Error
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "IOError", "ioError", "userError", "mkIOError",
            "isAlreadyExistsError", "isDoesNotExistError",
            "isAlreadyInUseError", "isFullError", "isEOFError",
            "isIllegalOperation", "isPermissionError", "isUserError",
            "ioeGetErrorType", "ioeGetLocation", "ioeGetErrorString",
            "ioeGetHandle", "ioeGetFileName",
            "tryIOError", "catchIOError",
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

    // ── Type.Reflection ───────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "TypeRep", "SomeTypeRep", "Typeable", "typeRep", "typeRepFingerprint",
            "rnfTypeRep", "eqTypeRep", "typeRepTyCon", "withTypeable", "pattern App",
            "pattern Con", "pattern Fun", "typeOf", "someTypeRep", "someTypeRepTyCon",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["TypeRep", "SomeTypeRep", "Typeable", "TyCon", "Module", "Fingerprint"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Type.Reflection".to_string(),
            exports_chirho,
        });
    }

    // ── Unsafe.Coerce ─────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("unsafeCoerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("unsafeCoerce#");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("UnsafeEquality", &["UnsafeRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Unsafe.Coerce".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.ForeignPtr (additional exports) ───────────────────────────
    // Already have Foreign.ForeignPtr, add GHC.ForeignPtr.Internal
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in ["ForeignPtr", "newForeignPtr", "withForeignPtr", "finalizeForeignPtr", "castForeignPtr", "plusForeignPtr"] {
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

    // ── GHC.Fingerprint ──────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in ["Fingerprint", "fingerprintData", "fingerprintString"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Fingerprint", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Fingerprint".to_string(),
            exports_chirho,
        });
        // Also alias as GHC.Fingerprint.Type
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Fingerprint.Type".to_string(),
            exports_chirho: {
                let mut e_chirho = IfaceExportsChirho::default();
                let (k_chirho, v_chirho) = mk_type_chirho("Fingerprint", &[]);
                e_chirho.types_chirho.insert(k_chirho, v_chirho);
                e_chirho
            },
        });
    }

    // ── GHC.Exception ────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "SomeException", "Exception", "toException", "fromException",
            "displayException", "throw", "throwIO", "ErrorCall",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["SomeException", "Exception", "ErrorCall"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exception".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Exception.Type".to_string(),
            exports_chirho,
        });
    }

    // ── GHC.IO.Exception ─────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "IOException", "IOError", "ioError", "userError",
            "BlockedIndefinitelyOnMVar", "BlockedIndefinitelyOnSTM",
            "AsyncException", "SomeAsyncException",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["IOException", "IOError", "AsyncException", "SomeAsyncException"] {
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
            "Array", "array", "listArray", "accumArray", "elems", "indices",
            "assocs", "bounds", "(!)", "(//)", "accum", "ixmap", "range",
            "index", "inRange", "rangeSize", "Ix",
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

    // ── Text.Printf ──────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "printf", "hPrintf", "PrintfArg", "HPrintfType", "PrintfType",
            "formatString", "formatInt", "formatFloat", "formatChar",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["PrintfArg", "PrintfType", "HPrintfType"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Printf".to_string(),
            exports_chirho,
        });
    }

    // ── Text.Read / Text.Show ────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Read", "read", "reads", "readParen", "readPrec", "readMaybe",
            "readEither", "lex", "ReadPrec", "ReadS",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Read", "ReadPrec", "ReadS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Read".to_string(),
            exports_chirho: exports_chirho.clone(),
        });

        let mut show_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Show", "show", "showsPrec", "showString", "showParen", "ShowS",
            "shows", "showChar", "showList",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            show_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Show", "ShowS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            show_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Show".to_string(),
            exports_chirho: show_exports_chirho,
        });
    }

    // ── Data.Fixed ───────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Fixed", "HasResolution", "resolution", "Pico", "Nano", "Micro",
            "Milli", "Centi", "Deci", "Uni", "E0", "E1", "E2", "E3",
            "E6", "E9", "E12", "showFixed", "mod'", "div'", "divMod'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Fixed", "HasResolution", "Pico", "Nano", "Micro", "Milli", "Centi", "Deci", "Uni"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Fixed".to_string(),
            exports_chirho,
        });
    }

    // ── Numeric / Numeric.Natural ────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "showInt", "showHex", "showOct", "readInt", "readHex", "readOct",
            "readDec", "readFloat", "readSigned", "Numeric", "fromRat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric".to_string(),
            exports_chirho,
        });

        let mut nat_exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("Natural");
        nat_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        nat_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho: nat_exports_chirho,
        });
    }

    // Data.Type.Equality (21 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho(":~:", &["Refl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("(:~~:)", &["HRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TestEquality", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "Refl", "HRefl", "castWith", "gcastWith", "apply", "inner", "outer",
            "sym", "trans", "testEquality",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Equality".to_string(),
            exports_chirho,
        });
    }

    // Control.Applicative (17 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Applicative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Alternative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("WrappedMonad", &["WrapMonad"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("WrappedArrow", &["WrapArrow"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ZipList", &["ZipList"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "pure", "liftA", "liftA2", "liftA3", "<*>", "*>", "<*",
            "empty", "<|>", "some", "many", "optional",
            "getConst", "Const", "ZipList", "getZipList",
            "WrapMonad", "unwrapMonad", "WrapArrow", "unwrapArrow",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Applicative".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Identity (12 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Identity", &["Identity"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Identity", "runIdentity"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Identity".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.ST (9 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ST", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["runST", "fixST", "stToIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.ST".to_string(),
            exports_chirho,
        });
    }

    // GHC.Base (7 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Semigroup", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Monoid", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Applicative", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Monad", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("NonEmpty", &["(:|)"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "id", "const", "flip", ".", "$", "&&", "||", "not",
            "map", "++", "foldr", "fmap", "<>", "mempty", "mappend", "mconcat",
            "pure", "return", ">>=", ">>", "=<<", "join", "ap",
            "otherwise", "error", "undefined", "seq", "oneShot",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Base".to_string(),
            exports_chirho,
        });
    }

    // Control.Arrow (6 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Arrow", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowChoice", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowApply", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowZero", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowPlus", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("ArrowLoop", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Kleisli", &["Kleisli"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "arr", "first", "second", "***", "&&&",
            ">>>", "<<<", "returnA",
            "left", "right", "|||", "+++",
            "app", "Kleisli", "runKleisli",
            "loop",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Arrow".to_string(),
            exports_chirho,
        });
    }

    // Data.Semigroup (used by some test files)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Semigroup", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Min", &["Min"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Max", &["Max"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("First", &["First"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Last", &["Last"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Arg", &["Arg"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "<>", "sconcat", "stimes",
            "Min", "getMin", "Max", "getMax",
            "First", "getFirst", "Last", "getLast",
            "Arg",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Semigroup".to_string(),
            exports_chirho,
        });
    }

    // Data.Monoid
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Monoid", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Dual", &["Dual"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Endo", &["Endo"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Sum", &["Sum"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Product", &["Product"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("All", &["All"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Any", &["Any"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Ap", &["Ap"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Alt", &["Alt"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "mempty", "mappend", "mconcat",
            "Dual", "getDual", "Endo", "appEndo",
            "Sum", "getSum", "Product", "getProduct",
            "All", "getAll", "Any", "getAny",
            "Ap", "getAp", "Alt", "getAlt",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Monoid".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor (common import)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Functor", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["fmap", "<$>", "<$", "$>", "void", "<&>"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Const
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Const", &["Const"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Const", "getConst"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Const".to_string(),
            exports_chirho,
        });
    }

    // Data.Functor.Compose
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compose", &["Compose"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Compose", "getCompose"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Functor.Compose".to_string(),
            exports_chirho,
        });
    }

    // Data.Void
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Void", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["absurd", "vacuous"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Void".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Bool (used in DataKinds tests)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("If", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Not", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Bool".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Ord (DataKinds type-level ordering)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Compare", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("OrdCond", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Ord".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack (commonly imported)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HasCallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("CallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SrcLoc", &["SrcLoc"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "callStack", "getCallStack", "prettyCallStack",
            "prettySrcLoc", "currentCallStack", "withFrozenCallStack",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Stack".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef (ST mutable references)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["newSTRef", "readSTRef", "writeSTRef", "modifySTRef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Type.Coercion
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Coercion", &["Coercion"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["Coercion", "coerceWith", "sym", "trans", "repr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Coercion".to_string(),
            exports_chirho,
        });
    }

    // Data.Foldable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Foldable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "fold", "foldMap", "foldMap'", "foldr", "foldl", "foldl'",
            "foldr'", "toList", "null", "length", "elem", "maximum",
            "minimum", "sum", "product", "and", "or", "any", "all",
            "concat", "concatMap", "find", "asum", "mapM_", "forM_",
            "sequenceA_", "sequence_", "traverse_", "for_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Foldable".to_string(),
            exports_chirho,
        });
    }

    // Data.Traversable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Traversable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "traverse", "sequenceA", "mapM", "sequence",
            "for", "forM", "mapAccumL", "mapAccumR", "fmapDefault", "foldMapDefault",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Traversable".to_string(),
            exports_chirho,
        });
    }

    // GHC.IO (internal module needed by some tests)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IO", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["unsafePerformIO", "unsafeInterleaveIO", "unsafeDupablePerformIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.IO".to_string(),
            exports_chirho,
        });
    }

    // System.IO.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["unsafePerformIO", "unsafeInterleaveIO", "unsafeDupablePerformIO", "unsafeFixIO"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.IO.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (some tests import this directly)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["newIORef", "readIORef", "writeIORef", "modifyIORef", "modifyIORef'", "atomicModifyIORef", "atomicModifyIORef'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // GHC.List (re-exports from GHC internal)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map", "filter", "head", "tail", "last", "init", "null", "length",
            "reverse", "foldl", "foldl'", "foldr", "foldr'", "scanl", "scanr",
            "iterate", "repeat", "replicate", "cycle", "take", "drop",
            "splitAt", "takeWhile", "dropWhile", "span", "break",
            "elem", "notElem", "lookup", "zip", "zip3", "zipWith", "zipWith3",
            "unzip", "unzip3", "lines", "words", "unlines", "unwords",
            "concat", "concatMap", "and", "or", "any", "all",
            "sum", "product", "maximum", "minimum",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.List".to_string(),
            exports_chirho,
        });
    }

    // ── Batch 5: additional commonly imported modules ──────────────

    // Data.Proxy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Proxy", &["Proxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("Proxy");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("asProxyTypeOf");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Proxy".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Typeable", "TypeRep", "Proxy"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "typeOf", "typeRep", "cast", "gcast", "eqT",
            "typeRepTyCon", "mkTyCon3", "mkTyConApp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Data.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Coercible", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("coerce");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Coerce".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Nat", "Symbol", "KnownNat", "KnownSymbol", "SomeNat", "SomeSymbol", "TypeError", "ErrorMessage"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "natVal", "natVal'", "symbolVal", "symbolVal'",
            "someNatVal", "someSymbolVal", "sameNat", "sameSymbol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats (re-exports from GHC.TypeLits plus extras)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Nat", "KnownNat", "SomeNat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["natVal", "natVal'", "someNatVal", "sameNat"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "sizeOf", "alignment", "peek", "poke", "peekElemOff",
            "pokeElemOff", "peekByteOff", "pokeByteOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Ptr", "FunPtr", "WordPtr", "IntPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "nullPtr", "castPtr", "plusPtr", "alignPtr", "minusPtr",
            "nullFunPtr", "castFunPtr", "freeHaskellFunPtr",
            "ptrToWordPtr", "wordPtrToPtr", "ptrToIntPtr", "intPtrToPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Control.Concurrent.MVar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("MVar", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newMVar", "newEmptyMVar", "takeMVar", "putMVar",
            "readMVar", "swapMVar", "tryTakeMVar", "tryPutMVar",
            "isEmptyMVar", "withMVar", "modifyMVar", "modifyMVar_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Concurrent.MVar".to_string(),
            exports_chirho,
        });
    }

    // Control.Monad.Trans.Class
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("MonadTrans", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("lift");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Control.Monad.Trans.Class".to_string(),
            exports_chirho,
        });
    }

    // Text.PrettyPrint
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Doc", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "text", "char", "int", "integer", "empty", "nest", "sep",
            "fsep", "hsep", "vcat", "hcat", "hang", "punctuate",
            "render", "parens", "brackets", "braces", "quotes",
            "doubleQuotes", "comma", "colon", "semi", "space",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.PrettyPrint".to_string(),
            exports_chirho,
        });
    }

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Data", "Typeable", "Constr", "DataType", "DataRep", "ConstrRep"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "toConstr", "gunfold", "gfoldl", "dataTypeOf",
            "mkConstr", "mkDataType", "constrIndex", "showConstr",
            "dataTypeConstrs", "dataTypeName", "constrType",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
    }

    // Data.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int", "Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Int".to_string(),
            exports_chirho,
        });
    }

    // Data.Map / Data.Map.Strict / Data.Map.Lazy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Map", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty", "singleton", "insert", "delete", "lookup",
            "member", "notMember", "findWithDefault",
            "union", "unionWith", "intersection", "difference",
            "map", "mapWithKey", "filter", "filterWithKey",
            "foldlWithKey'", "foldrWithKey", "toList", "fromList",
            "toAscList", "toDescList", "elems", "keys", "size", "null",
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

    // Data.Set
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Set", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty", "singleton", "insert", "delete", "member",
            "notMember", "size", "null", "union", "intersection",
            "difference", "map", "filter", "foldl'", "foldr",
            "toList", "fromList", "toAscList", "toDescList", "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Set".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Generic", "Generic1", "Rep", "Rep1",
            "V1", "U1", "K1", "M1", "Par1",
            "Rec0", "Rec1", "D1", "C1", "S1",
            "Meta", "Datatype", "Constructor", "Selector",
            "Fixity", "Associativity", "SourceUnpackedness", "SourceStrictness",
            "DecidedStrictness",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["from", "to", "from1", "to1", "datatypeName", "moduleName", "packageName", "conName", "conFixity", "conIsRecord", "selName"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Generics".to_string(),
            exports_chirho,
        });
    }

    // ── Batch 5b: missing utility modules ──────────────────────────

    // Debug.Trace
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "trace", "traceShow", "traceShowId", "traceIO",
            "traceM", "traceShowM", "traceStack",
            "traceEvent", "traceEventIO", "traceMarker", "traceMarkerIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Debug.Trace".to_string(),
            exports_chirho,
        });
    }

    // Data.Bits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Bits", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("FiniteBits", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "(.&.)", "(.|.)", "xor", "complement", "shift", "rotate",
            "setBit", "clearBit", "complementBit", "testBit", "bit",
            "zeroBits", "shiftL", "shiftR", "rotateL", "rotateR",
            "popCount", "bitSize", "bitSizeMaybe", "isSigned",
            "unsafeShiftL", "unsafeShiftR",
            "finiteBitSize", "countLeadingZeros", "countTrailingZeros",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Bits".to_string(),
            exports_chirho,
        });
    }

    // Data.Complex
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Complex", &[":+"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[":+", "realPart", "imagPart", "mkPolar", "cis", "polar", "magnitude", "phase", "conjugate"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Complex".to_string(),
            exports_chirho,
        });
    }

    // Data.Ratio
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Ratio", &[":%"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Rational", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["(%)", "numerator", "denominator", "approxRational"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ratio".to_string(),
            exports_chirho,
        });
    }

    // Data.Fixed
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Fixed", "HasResolution", "E0", "E1", "E2", "E3", "E6", "E9", "E12", "Uni", "Deci", "Centi", "Milli", "Micro", "Nano", "Pico"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["resolution", "showFixed", "mod'", "divMod'", "div'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Fixed".to_string(),
            exports_chirho,
        });
    }

    // Data.Unique
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Unique", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["newUnique", "hashUnique"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Unique".to_string(),
            exports_chirho,
        });
    }

    // System.Exit
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ExitCode", &["ExitSuccess", "ExitFailure"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["ExitSuccess", "ExitFailure", "exitWith", "exitSuccess", "exitFailure", "die"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "System.Exit".to_string(),
            exports_chirho,
        });
    }

    // Data.Dynamic
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Dynamic", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["toDyn", "fromDyn", "fromDynamic", "dynTypeRep", "dynApply", "dynApp"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Dynamic".to_string(),
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
            name_chirho: "Control.DeepSeq".to_string(),
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
            "empty", "singleton", "(<|)", "(|>)", "(><)",
            "fromList", "length", "null",
            "index", "adjust", "update", "take", "drop", "splitAt",
            "viewl", "viewr", "filter", "sort", "reverse",
            "zip", "zipWith", "unzip", "replicate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Sequence".to_string(),
            exports_chirho,
        });
    }

    // Data.IntMap
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IntMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty", "singleton", "insert", "delete", "lookup",
            "member", "size", "null", "union", "intersection", "difference",
            "map", "mapWithKey", "filter", "filterWithKey",
            "foldlWithKey'", "foldrWithKey", "toList", "fromList",
            "toAscList", "keys", "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntMap".to_string(),
            exports_chirho,
        });
    }

    // Data.IntSet
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IntSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty", "singleton", "insert", "delete", "member",
            "notMember", "size", "null", "union", "intersection", "difference",
            "filter", "foldl'", "foldr", "toList", "fromList", "toAscList", "elems",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.IntSet".to_string(),
            exports_chirho,
        });
    }

    // Data.HashMap.Strict (from unordered-containers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HashMap", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "empty", "singleton", "insert", "delete", "lookup",
            "member", "size", "null", "union", "intersection", "difference",
            "map", "mapWithKey", "filter", "filterWithKey",
            "foldlWithKey'", "foldrWithKey", "toList", "fromList", "keys", "elems",
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
            "empty", "singleton", "insert", "delete", "member",
            "size", "null", "union", "intersection", "difference",
            "filter", "foldl'", "foldr", "toList", "fromList",
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

    // GHC.Records
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("HasField", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho("getField");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Records".to_string(),
            exports_chirho,
        });
    }

    // GHC.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["naturalToInteger", "naturalFromInteger", "naturalToWord", "wordToNatural", "intToNatural", "minusNaturalMaybe"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Natural".to_string(),
            exports_chirho,
        });
    }

    // GHC.Num
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Num", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["+", "-", "*", "negate", "abs", "signum", "fromInteger", "subtract"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num".to_string(),
            exports_chirho,
        });
    }

    // GHC.Real
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Integral", "Fractional", "Real", "RealFrac", "Ratio", "Rational"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "toInteger", "toRational", "fromIntegral", "realToFrac",
            "quot", "rem", "div", "mod", "quotRem", "divMod",
            "(%)", "numerator", "denominator",
            "ceiling", "floor", "round", "truncate", "properFraction",
            "recip", "fromRational",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Real".to_string(),
            exports_chirho,
        });
    }

    // GHC.Float
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Float", "Double", "Floating", "RealFloat"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "pi", "exp", "log", "sqrt", "sin", "cos", "tan",
            "asin", "acos", "atan", "sinh", "cosh", "tanh",
            "asinh", "acosh", "atanh", "(**)", "logBase",
            "floatRadix", "floatDigits", "floatRange",
            "decodeFloat", "encodeFloat", "exponent", "significand", "scaleFloat",
            "isNaN", "isInfinite", "isDenormalized", "isNegativeZero", "isIEEE",
            "atan2", "float2Double", "double2Float",
            "int2Double", "int2Float",
            "showFloat", "showEFloat", "showFFloat", "showGFloat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Float".to_string(),
            exports_chirho,
        });
    }

    // GHC.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Show", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["show", "showsPrec", "showString", "showChar", "showParen", "shows", "showList__"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Show".to_string(),
            exports_chirho,
        });
    }

    // GHC.Read
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Read", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["read", "reads", "readPrec", "readList", "readParen", "lex"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Read".to_string(),
            exports_chirho,
        });
    }

    // GHC.Enum
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Enum", "Bounded"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["succ", "pred", "toEnum", "fromEnum", "enumFrom", "enumFromThen", "enumFromTo", "enumFromThenTo", "minBound", "maxBound"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Enum".to_string(),
            exports_chirho,
        });
    }

    modules_chirho
}

/// Collect all definitions (types + values) from a module's declarations.
fn collect_all_definitions_chirho(module_chirho: &ModuleChirho) -> IfaceExportsChirho {
    let mut exports_chirho = IfaceExportsChirho::default();

    for decl_chirho in &module_chirho.decls_chirho {
        match decl_chirho {
            DeclChirho::DataDeclChirho {
                name_chirho,
                constructors_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = name_chirho.text_chirho().to_string();
                let con_names_chirho: Vec<String> = constructors_chirho
                    .iter()
                    .map(|c_chirho| con_decl_name_chirho(c_chirho).to_string())
                    .collect();

                // Export each constructor as a value
                for cn_chirho in &con_names_chirho {
                    exports_chirho.values_chirho.insert(
                        cn_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: cn_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: con_names_chirho,
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::NewtypeDeclChirho {
                name_chirho,
                constructor_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = name_chirho.text_chirho().to_string();
                let con_name_chirho = con_decl_name_chirho(constructor_chirho).to_string();

                exports_chirho.values_chirho.insert(
                    con_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: con_name_chirho.clone(),
                        span_chirho: *span_chirho,
                    },
                );

                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![con_name_chirho],
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::TypeAliasDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let type_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.types_chirho.insert(
                    type_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: type_name_chirho,
                        constructors_chirho: vec![],
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::ClassDeclChirho {
                name_chirho,
                methods_chirho,
                span_chirho,
                ..
            } => {
                let class_name_chirho = name_chirho.text_chirho().to_string();
                let method_names_chirho: Vec<String> = methods_chirho
                    .iter()
                    .map(|m_chirho| m_chirho.name_chirho.text_chirho().to_string())
                    .collect();

                // Export each method as a value
                for mn_chirho in &method_names_chirho {
                    exports_chirho.values_chirho.insert(
                        mn_chirho.clone(),
                        IfaceValueChirho {
                            name_chirho: mn_chirho.clone(),
                            span_chirho: *span_chirho,
                        },
                    );
                }

                exports_chirho.types_chirho.insert(
                    class_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: class_name_chirho,
                        constructors_chirho: vec![],
                        methods_chirho: method_names_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::FunBindChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let fn_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.values_chirho.insert(
                    fn_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: fn_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::PatBindChirho { .. } => {
                // Pattern bindings are not exported by name
            }
            DeclChirho::TypeSigChirho { .. }
            | DeclChirho::InstanceDeclChirho { .. }
            | DeclChirho::FixityDeclChirho { .. }
            | DeclChirho::DefaultDeclChirho { .. }
            | DeclChirho::ForeignDeclChirho { .. } => {}
            DeclChirho::TypeFamilyDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let family_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.types_chirho.insert(
                    family_name_chirho.clone(),
                    IfaceTypeChirho {
                        name_chirho: family_name_chirho,
                        constructors_chirho: vec![],
                        methods_chirho: vec![],
                        span_chirho: *span_chirho,
                    },
                );
            }
            DeclChirho::TypeFamilyInstanceDeclChirho { .. } => {
                // Type family instances don't introduce new names
            }
            DeclChirho::SpliceDeclChirho { .. } => {
                // TH splice declarations don't directly export names;
                // they must be evaluated to generate concrete declarations first.
            }
            DeclChirho::StandaloneDerivingDeclChirho { .. } => {
                // Standalone deriving is handled by the deriving pass, not exports.
            }
            DeclChirho::PatSynDeclChirho {
                name_chirho,
                span_chirho,
                ..
            } => {
                let syn_name_chirho = name_chirho.text_chirho().to_string();
                exports_chirho.values_chirho.insert(
                    syn_name_chirho.clone(),
                    IfaceValueChirho {
                        name_chirho: syn_name_chirho,
                        span_chirho: *span_chirho,
                    },
                );
            }
        }
    }

    exports_chirho
}

/// Apply an explicit export list to filter the module's definitions.
/// `module_chirho` is the parsed module (for checking imports of re-exported modules).
/// `imported_ifaces_chirho` provides the interfaces of imported modules for re-exports.
fn filter_exports_chirho(
    all_chirho: &IfaceExportsChirho,
    specs_chirho: &[ExportSpecChirho],
    module_chirho: &ModuleChirho,
    imported_ifaces_chirho: &[ModuleIfaceChirho],
) -> IfaceExportsChirho {
    let mut result_chirho = IfaceExportsChirho::default();

    for spec_chirho in specs_chirho {
        match spec_chirho {
            ExportSpecChirho::VarChirho(name_chirho) => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(val_chirho) = all_chirho.values_chirho.get(text_chirho) {
                    result_chirho
                        .values_chirho
                        .insert(text_chirho.to_string(), val_chirho.clone());
                }
                // A bare name in an export list can also refer to a type
                if let Some(ty_chirho) = all_chirho.types_chirho.get(text_chirho) {
                    result_chirho.types_chirho.insert(
                        text_chirho.to_string(),
                        IfaceTypeChirho {
                            name_chirho: ty_chirho.name_chirho.clone(),
                            constructors_chirho: vec![], // bare name = no constructors
                            methods_chirho: vec![],
                            span_chirho: ty_chirho.span_chirho,
                        },
                    );
                }
            }
            ExportSpecChirho::TyConChirho {
                name_chirho,
                members_chirho,
            } => {
                let text_chirho = name_chirho.text_chirho();
                if let Some(ty_chirho) = all_chirho.types_chirho.get(text_chirho) {
                    let (cons_chirho, methods_chirho) = match members_chirho {
                        ExportMembersChirho::AllChirho => (
                            ty_chirho.constructors_chirho.clone(),
                            ty_chirho.methods_chirho.clone(),
                        ),
                        ExportMembersChirho::SomeChirho(names_chirho) => {
                            let selected_chirho: Vec<String> = names_chirho
                                .iter()
                                .map(|n_chirho| n_chirho.text_chirho().to_string())
                                .collect();
                            let sel_cons_chirho: Vec<String> = ty_chirho
                                .constructors_chirho
                                .iter()
                                .filter(|c_chirho| selected_chirho.contains(c_chirho))
                                .cloned()
                                .collect();
                            let sel_methods_chirho: Vec<String> = ty_chirho
                                .methods_chirho
                                .iter()
                                .filter(|m_chirho| selected_chirho.contains(m_chirho))
                                .cloned()
                                .collect();
                            (sel_cons_chirho, sel_methods_chirho)
                        }
                        ExportMembersChirho::NoneChirho => (vec![], vec![]),
                    };

                    // Export the selected constructors/methods as values
                    for cn_chirho in &cons_chirho {
                        if let Some(val_chirho) = all_chirho.values_chirho.get(cn_chirho) {
                            result_chirho
                                .values_chirho
                                .insert(cn_chirho.clone(), val_chirho.clone());
                        }
                    }
                    for mn_chirho in &methods_chirho {
                        if let Some(val_chirho) = all_chirho.values_chirho.get(mn_chirho) {
                            result_chirho
                                .values_chirho
                                .insert(mn_chirho.clone(), val_chirho.clone());
                        }
                    }

                    result_chirho.types_chirho.insert(
                        text_chirho.to_string(),
                        IfaceTypeChirho {
                            name_chirho: ty_chirho.name_chirho.clone(),
                            constructors_chirho: cons_chirho,
                            methods_chirho,
                            span_chirho: ty_chirho.span_chirho,
                        },
                    );
                }
            }
            ExportSpecChirho::ModuleChirho(re_export_name_chirho) => {
                let target_mod_chirho = re_export_name_chirho.text_chirho();
                // Check the module is actually imported.
                let is_imported_chirho = module_chirho.imports_chirho.iter().any(|imp_chirho| {
                    imp_chirho.module_chirho.text_chirho() == target_mod_chirho
                });
                // `module M` in the export list of module M itself means
                // "export all local definitions" — this is the self-re-export pattern.
                let is_self_chirho = module_chirho.name_chirho.text_chirho() == target_mod_chirho;

                if is_self_chirho {
                    // Export all local definitions.
                    for (k_chirho, v_chirho) in &all_chirho.values_chirho {
                        result_chirho
                            .values_chirho
                            .insert(k_chirho.clone(), v_chirho.clone());
                    }
                    for (k_chirho, v_chirho) in &all_chirho.types_chirho {
                        result_chirho
                            .types_chirho
                            .insert(k_chirho.clone(), v_chirho.clone());
                    }
                } else if is_imported_chirho {
                    // Find the matching interface and re-export all its names.
                    if let Some(iface_chirho) = imported_ifaces_chirho
                        .iter()
                        .find(|i_chirho| i_chirho.name_chirho == target_mod_chirho)
                    {
                        for (k_chirho, v_chirho) in &iface_chirho.exports_chirho.values_chirho {
                            result_chirho
                                .values_chirho
                                .insert(k_chirho.clone(), v_chirho.clone());
                        }
                        for (k_chirho, v_chirho) in &iface_chirho.exports_chirho.types_chirho {
                            result_chirho
                                .types_chirho
                                .insert(k_chirho.clone(), v_chirho.clone());
                        }
                    }
                }
                // If not imported and not self, silently skip (could warn).
            }
        }
    }

    result_chirho
}

fn con_decl_name_chirho(decl_chirho: &ConDeclChirho) -> &str {
    match decl_chirho {
        ConDeclChirho::OrdinaryChirho { name_chirho, .. }
        | ConDeclChirho::RecordChirho { name_chirho, .. }
        | ConDeclChirho::GadtChirho { name_chirho, .. } => name_chirho.text_chirho(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use haskelujah_ast_chirho::name_chirho::{NameChirho, RawNameChirho};

    fn mk_name_chirho(s_chirho: &str) -> NameChirho {
        NameChirho::RawChirho(RawNameChirho::unqualified_chirho(
            s_chirho,
            SpanChirho::DUMMY_CHIRHO,
        ))
    }

    fn mk_module_chirho(
        name_chirho: &str,
        exports_chirho: Option<Vec<ExportSpecChirho>>,
        decls_chirho: Vec<DeclChirho>,
    ) -> ModuleChirho {
        ModuleChirho {
            name_chirho: mk_name_chirho(name_chirho),
            exports_chirho,
            imports_chirho: vec![],
            decls_chirho,
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        }
    }

    #[test]
    fn export_all_when_no_export_list_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            None, // no export list = export everything
            vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: mk_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: mk_name_chirho("Red"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                        ConDeclChirho::OrdinaryChirho {
                            name_chirho: mk_name_chirho("Blue"),
                            fields_chirho: vec![],
                            span_chirho: SpanChirho::DUMMY_CHIRHO,
                        },
                    ],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: mk_name_chirho("paint"),
                    matches_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert_eq!(iface_chirho.name_chirho, "Lib");
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Color"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Blue"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("paint"));
    }

    #[test]
    fn export_list_filters_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            Some(vec![
                // Export Color with all constructors
                ExportSpecChirho::TyConChirho {
                    name_chirho: mk_name_chirho("Color"),
                    members_chirho: ExportMembersChirho::AllChirho,
                },
                // Export paint function
                ExportSpecChirho::VarChirho(mk_name_chirho("paint")),
                // Do NOT export helper
            ]),
            vec![
                DeclChirho::DataDeclChirho {
                    name_chirho: mk_name_chirho("Color"),
                    type_vars_chirho: vec![],
                    constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Red"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    }],
                    deriving_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: mk_name_chirho("paint"),
                    matches_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                DeclChirho::FunBindChirho {
                    name_chirho: mk_name_chirho("helper"),
                    matches_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            ],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Color"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("paint"));
        // helper is NOT exported
        assert!(!iface_chirho.exports_chirho.values_chirho.contains_key("helper"));
    }

    #[test]
    fn export_type_without_constructors_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            Some(vec![ExportSpecChirho::TyConChirho {
                name_chirho: mk_name_chirho("Color"),
                members_chirho: ExportMembersChirho::NoneChirho,
            }]),
            vec![DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Red"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Color"));
        // Red is NOT exported — only the type name
        assert!(!iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        let color_ty_chirho = &iface_chirho.exports_chirho.types_chirho["Color"];
        assert!(color_ty_chirho.constructors_chirho.is_empty());
    }

    #[test]
    fn export_some_constructors_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            Some(vec![ExportSpecChirho::TyConChirho {
                name_chirho: mk_name_chirho("Color"),
                members_chirho: ExportMembersChirho::SomeChirho(vec![mk_name_chirho("Red")]),
            }]),
            vec![DeclChirho::DataDeclChirho {
                name_chirho: mk_name_chirho("Color"),
                type_vars_chirho: vec![],
                constructors_chirho: vec![
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Red"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                    ConDeclChirho::OrdinaryChirho {
                        name_chirho: mk_name_chirho("Blue"),
                        fields_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                ],
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Red"));
        assert!(!iface_chirho.exports_chirho.values_chirho.contains_key("Blue"));
        let ty_chirho = &iface_chirho.exports_chirho.types_chirho["Color"];
        assert_eq!(ty_chirho.constructors_chirho, vec!["Red"]);
    }

    #[test]
    fn class_methods_exported_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            None,
            vec![DeclChirho::ClassDeclChirho {
                context_chirho: vec![],
                name_chirho: mk_name_chirho("Show"),
                type_vars_chirho: vec![],
                methods_chirho: vec![haskelujah_ast_chirho::decl_chirho::ClassMethodChirho {
                    name_chirho: mk_name_chirho("show"),
                    ty_chirho: haskelujah_ast_chirho::ty_chirho::TypeChirho::VarChirho(
                        mk_name_chirho("a"),
                    ),
                    default_chirho: None,
                    default_sig_chirho: None,
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                }],
                associated_tfs_chirho: vec![],
                fundeps_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Show"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("show"));
        assert_eq!(
            iface_chirho.exports_chirho.types_chirho["Show"].methods_chirho,
            vec!["show"]
        );
    }

    #[test]
    fn newtype_exported_chirho() {
        let module_chirho = mk_module_chirho(
            "Lib",
            None,
            vec![DeclChirho::NewtypeDeclChirho {
                name_chirho: mk_name_chirho("Wrapper"),
                type_vars_chirho: vec![],
                constructor_chirho: ConDeclChirho::OrdinaryChirho {
                    name_chirho: mk_name_chirho("Wrap"),
                    fields_chirho: vec![],
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
                deriving_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
        );

        let iface_chirho = build_iface_chirho(&module_chirho);
        assert!(iface_chirho.exports_chirho.types_chirho.contains_key("Wrapper"));
        assert!(iface_chirho.exports_chirho.values_chirho.contains_key("Wrap"));
    }

    #[test]
    fn module_re_export_chirho() {
        // module Reexporter (module Inner) where
        // import Inner
        // extra = 42
        use haskelujah_ast_chirho::module_chirho::ImportDeclChirho;

        // Simulate the Inner module interface.
        let inner_iface_chirho = ModuleIfaceChirho {
            name_chirho: "Inner".to_string(),
            exports_chirho: {
                let mut e_chirho = IfaceExportsChirho::default();
                e_chirho.values_chirho.insert(
                    "innerFn".to_string(),
                    IfaceValueChirho {
                        name_chirho: "innerFn".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho.types_chirho.insert(
                    "InnerType".to_string(),
                    IfaceTypeChirho {
                        name_chirho: "InnerType".to_string(),
                        constructors_chirho: vec!["MkInner".to_string()],
                        methods_chirho: vec![],
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho.values_chirho.insert(
                    "MkInner".to_string(),
                    IfaceValueChirho {
                        name_chirho: "MkInner".to_string(),
                        span_chirho: SpanChirho::DUMMY_CHIRHO,
                    },
                );
                e_chirho
            },
        };

        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("Reexporter"),
            exports_chirho: Some(vec![
                // Re-export everything from Inner.
                ExportSpecChirho::ModuleChirho(mk_name_chirho("Inner")),
            ]),
            imports_chirho: vec![ImportDeclChirho {
                module_chirho: mk_name_chirho("Inner"),
                qualified_chirho: false,
                alias_chirho: None,
                spec_chirho: None,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("extra"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let iface_chirho =
            build_iface_with_imports_chirho(&module_chirho, &[inner_iface_chirho]);

        // Re-exported names from Inner should be present.
        assert!(
            iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("innerFn"),
            "innerFn should be re-exported"
        );
        assert!(
            iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("MkInner"),
            "MkInner constructor should be re-exported"
        );
        assert!(
            iface_chirho
                .exports_chirho
                .types_chirho
                .contains_key("InnerType"),
            "InnerType should be re-exported"
        );
        // `extra` is NOT in the export list (only `module Inner` is).
        assert!(
            !iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("extra"),
            "extra should NOT be exported (not in export list)"
        );
    }

    #[test]
    fn self_re_export_chirho() {
        // module Lib (module Lib) where
        // foo = 1
        let module_chirho = ModuleChirho {
            name_chirho: mk_name_chirho("Lib"),
            exports_chirho: Some(vec![
                ExportSpecChirho::ModuleChirho(mk_name_chirho("Lib")),
            ]),
            imports_chirho: vec![],
            decls_chirho: vec![DeclChirho::FunBindChirho {
                name_chirho: mk_name_chirho("foo"),
                matches_chirho: vec![],
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            }],
            extensions_chirho: vec![],
            inline_pragmas_chirho: std::collections::HashMap::new(),
            specialize_pragmas_chirho: std::collections::HashMap::new(),
            foreign_exports_chirho: vec![],
            deriving_via_chirho: vec![],
            span_chirho: SpanChirho::DUMMY_CHIRHO,
        };

        let iface_chirho = build_iface_chirho(&module_chirho);
        // Self re-export means export all local definitions.
        assert!(
            iface_chirho
                .exports_chirho
                .values_chirho
                .contains_key("foo"),
            "foo should be exported via self re-export"
        );
    }
}
