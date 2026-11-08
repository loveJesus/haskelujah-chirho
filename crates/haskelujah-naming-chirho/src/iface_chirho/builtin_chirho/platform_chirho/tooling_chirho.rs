// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `Language.Haskell`, test frameworks, benchmarks and Haskelujah's own modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `Language.Haskell`, test frameworks, benchmarks and Haskelujah's own modules.
pub(super) fn push_tooling_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Gauge.Main / Criterion.Main (benchmarking)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "defaultMain",
            "bench",
            "bgroup",
            "nf",
            "whnf",
            "env",
            "nfIO",
            "whnfIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Benchmark", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_name_chirho in &["Gauge.Main", "Gauge", "Criterion.Main", "Criterion"] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Language.Haskell.TH — Template Haskell types and Q monad
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            // Q monad
            "Q",
            "runQ",
            "runIO",
            "newName",
            "lookupTypeName",
            "lookupValueName",
            "reify",
            "reifyRoles",
            "reifyAnnotations",
            "reifyInstances",
            "reifyConStrictness",
            "reifyFixity",
            "reifyType",
            "isInstance",
            "pprint",
            "pprExp",
            "pprPat",
            "pprDec",
            "pprType",
            "location",
            // Syntax construction
            "mkName",
            "nameBase",
            "nameModule",
            "ConT",
            "VarT",
            "AppT",
            "AppKindT",
            "SigT",
            "ForallT",
            "InfixT",
            "ParensT",
            "TupleT",
            "UnboxedTupleT",
            "ArrowT",
            "ListT",
            "StarT",
            "appKindT",
            "appK",
            "arrowK",
            "starK",
            "PlainTV",
            "KindedTV",
            "SpecifiedSpec",
            "InferredSpec",
            "plainTV",
            "kindedTV",
            "plainTVSpecified",
            "plainTVInferred",
            "kindedTVSpecified",
            "kindedTVInferred",
            "kindedTVReq",
            "elimTV",
            "tvName",
            "tvKind",
            // Expression constructors
            "litE",
            "varE",
            "conE",
            "appE",
            "appsE",
            "infixE",
            "infixApp",
            "lamE",
            "lam1E",
            "tupE",
            "unboxedTupE",
            "listE",
            "sigE",
            "recConE",
            "recUpdE",
            "letE",
            "caseE",
            "doE",
            "condE",
            "compE",
            "arithSeqE",
            "fromE",
            "fromThenE",
            "fromToE",
            "fromThenToE",
            "stringE",
            "charL",
            "stringL",
            "integerL",
            "rationalL",
            "intPrimL",
            "wordPrimL",
            // Data constructors
            "AppE",
            "ConE",
            "VarE",
            "LitE",
            "InfixE",
            "LamE",
            "TupE",
            "ListE",
            "SigE",
            // Name helpers
            "tupleDataName",
            "unboxedTupleDataName",
            "tupleTypeName",
            "unboxedTupleTypeName",
            "mkNameG_v",
            "mkNameG_tc",
            "mkNameG_d",
            // Pattern constructors
            "litP",
            "varP",
            "conP",
            "tupP",
            "listP",
            "wildP",
            "asP",
            "sigP",
            // Type constructors
            "conT",
            "varT",
            "appT",
            "arrowT",
            "listT",
            "tupleT",
            "sigT",
            "forallT",
            "implicitParamT",
            "appKindT",
            // Declaration constructors
            "funD",
            "valD",
            "dataD",
            "newtypeD",
            "tySynD",
            "classD",
            "instanceD",
            "sigD",
            "pragInlD",
            "pragLineD",
            "derivClause",
            "defaultFixity",
            // Clause and body
            "clause",
            "normalB",
            "guardedB",
            // Literals
            "integerL",
            "rationalL",
            "charL",
            "stringL",
            // Lift class
            "lift",
            "liftTyped",
            // Misc
            "reportError",
            "reportWarning",
            "recover",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Exp",
            "Pat",
            "Type",
            "Dec",
            "Body",
            "Clause",
            "Lit",
            "Name",
            "Info",
            "Loc",
            "Range",
            "Guard",
            "Stmt",
            "Match",
            "Con",
            "Strict",
            "FunDep",
            "Kind",
            "Pred",
            "Cxt",
            "TyVarBndr",
            "TyVarBndrUnit",
            "TyVarBndrSpec",
            "TyVarBndrVis",
            "Specificity",
            "TypeQ",
            "ExpQ",
            "PatQ",
            "DecQ",
            "DecsQ",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["template-haskell"]),
            name_chirho: "Language.Haskell.TH".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Sub-modules that re-export from the main TH interface
        for sub_chirho in &["Language.Haskell.TH.Ppr", "Language.Haskell.TH.PprLib"] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: package_exposure_chirho(&["template-haskell"]),
                name_chirho: sub_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Language.Haskell.TH.Syntax
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Lift",
            "lift",
            "liftTyped",
            "mkName",
            "nameBase",
            "mkNameG_v",
            "mkNameG_tc",
            "mkNameG_d",
            "ConT",
            "VarT",
            "AppT",
            "SigT",
            "ForallT",
            "InfixT",
            "ParensT",
            "TupleT",
            "UnboxedTupleT",
            "ArrowT",
            "ListT",
            "StarT",
            "PlainTV",
            "KindedTV",
            "SpecifiedSpec",
            "InferredSpec",
            "Q",
            "runQ",
            "runIO",
            "newName",
            "lookupTypeName",
            "lookupValueName",
            "reify",
            "implicitParamT",
            "derivClause",
            "pragLineD",
            "defaultFixity",
            "appKindT",
            "plainTV",
            "kindedTV",
            "plainTVSpecified",
            "plainTVInferred",
            "kindedTVSpecified",
            "kindedTVInferred",
            "kindedTVReq",
            "elimTV",
            "tvName",
            "tvKind",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (name_chirho, ctors_chirho) in [
            ("Name", &[][..]),
            ("Exp", &[][..]),
            ("Pat", &[][..]),
            (
                "Type",
                &[
                    "ForallT",
                    "AppT",
                    "AppKindT",
                    "SigT",
                    "VarT",
                    "ConT",
                    "InfixT",
                    "ParensT",
                    "TupleT",
                    "UnboxedTupleT",
                    "ArrowT",
                    "ListT",
                    "StarT",
                ][..],
            ),
            ("Dec", &[][..]),
            ("Lit", &[][..]),
            ("Kind", &[][..]),
            ("Pred", &[][..]),
            ("Cxt", &[][..]),
            ("TyVarBndr", &["PlainTV", "KindedTV"][..]),
            ("TyVarBndrUnit", &["PlainTV", "KindedTV"][..]),
            ("TyVarBndrSpec", &["PlainTV", "KindedTV"][..]),
            ("TyVarBndrVis", &["PlainTV", "KindedTV"][..]),
            ("Specificity", &["SpecifiedSpec", "InferredSpec"][..]),
            ("TypeQ", &[][..]),
            ("ExpQ", &[][..]),
            ("PatQ", &[][..]),
            ("DecQ", &[][..]),
            ("DecsQ", &[][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, ctors_chirho);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["template-haskell"]),
            name_chirho: "Language.Haskell.TH.Syntax".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Quote
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["quoteExp", "quotePat", "quoteType", "quoteDec"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("QuasiQuoter", &["QuasiQuoter"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["template-haskell"]),
            name_chirho: "Language.Haskell.TH.Quote".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Lib
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "litE", "varE", "conE", "appE", "lamE", "tupE", "listE", "litP", "varP", "conP",
            "tupP", "wildP", "sigP", "conT", "varT", "appT", "appKindT", "appK", "arrowT",
            "arrowK", "starK", "funD", "valD", "sigD", "clause", "normalB",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["template-haskell"]),
            name_chirho: "Language.Haskell.TH.Lib".to_string(),
            exports_chirho,
        });
    }

    // Language.Haskell.TH.Datatype (th-abstraction)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "reifyDatatype",
            "reifyConstructor",
            "reifyRecord",
            "normalizeInfo",
            "normalizeDec",
            "normalizeCon",
            "datatypeName",
            "datatypeVars",
            "datatypeCons",
            "datatypeInstTypes",
            "datatypeVariant",
            "constructorName",
            "constructorFields",
            "constructorStrictness",
            "constructorVariant",
            "constructorVars",
            "resolveTypeSynonyms",
            "quantifyType",
            "freeVariables",
            "freshenFreeVariables",
            "unifyTypes",
            "applySubstitution",
            "freeVariablesWellScoped",
            "tvName",
            "tvKind",
            "datatypeReturnKind",
            "starK",
            "isStarOrConstraint",
            "equalPred",
            "classPred",
            "asEqualPred",
            "asClassPred",
            "constructorType",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "DatatypeInfo",
            "ConstructorInfo",
            "DatatypeVariant",
            "ConstructorVariant",
            "FieldStrictness",
            "Unpackedness",
            "Strictness",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Language.Haskell.TH.Datatype".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Also expose as TyVarBndr sub-module
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Language.Haskell.TH.Datatype.TyVarBndr".to_string(),
            exports_chirho,
        });
    }

    // Test.HUnit.Lang (for HUnit internals)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Assertion",
            "assertFailure",
            "assertEqual",
            "assertBool",
            "HUnitFailure",
            "FailureReason",
            "formatFailureReason",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["HUnitFailure", "FailureReason"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Test.HUnit.Lang".to_string(),
            exports_chirho,
        });
    }

    // Test.Hspec (for package test-suite frontiers)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["hspec", "describe", "it", "shouldBe"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Spec", "Expectation"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Test.Hspec".to_string(),
            exports_chirho,
        });
    }

    // Test.Tasty / Test.Tasty.Options (tasty package)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "safeRead",
            "lookupOption",
            "Option",
            "defaultValue",
            "parseValue",
            "optionName",
            "optionHelp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, mut v_chirho) = mk_type_chirho("IsOption", &[]);
        v_chirho.methods_chirho = vec![
            "defaultValue".to_string(),
            "parseValue".to_string(),
            "optionName".to_string(),
            "optionHelp".to_string(),
        ];
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("OptionSet", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("OptionDescription", &["Option"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Test.Tasty.Options".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "defaultMain",
            "defaultMainWithIngredients",
            "testGroup",
            "includingOptions",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["TestTree", "Ingredient"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Test.Tasty".to_string(),
            exports_chirho,
        });
    }

    // ── Haskelujah stdlib (batteries included) ──────────────────────
    // These match the modules in stdlib-chirho/Haskelujah/*.hs
    for (mod_name_chirho, exports_list_chirho) in &[
        (
            "Haskelujah.JSON",
            &[
                "Value", "Object", "Array", "String", "Number", "Bool", "Null", "encode", "object",
                "array", ".=",
            ][..],
        ),
        (
            "Haskelujah.Test",
            &[
                "Test",
                "test",
                "runTests",
                "assertEqual",
                "assertBool",
                "assertFailure",
            ],
        ),
        (
            "Haskelujah.Args",
            &["getArgs", "getFlag", "getOption", "getPositional"],
        ),
        (
            "Haskelujah.HTTP",
            &["Response", "get", "post", "statusCode", "body", "headers"],
        ),
        (
            "Haskelujah.Prelude",
            &["trim", "words'", "unwords'", "groupBy'", "chunksOf", "nub'"],
        ),
        (
            "Haskelujah.File",
            &[
                "readFileText",
                "writeFileText",
                "appendFileText",
                "fileExists",
                "listDirectory",
            ],
        ),
        (
            "Haskelujah.Text",
            &[
                "toLower",
                "toUpper",
                "capitalize",
                "contains",
                "startsWith",
                "endsWith",
                "splitOn",
                "lines'",
                "unlines'",
                "padLeft",
                "padRight",
                "center",
            ],
        ),
        (
            "Haskelujah.Map",
            &[
                "Map",
                "empty",
                "singleton",
                "fromList",
                "toList",
                "insert",
                "lookup",
                "delete",
                "member",
                "size",
                "keys",
                "elems",
                "mapValues",
                "filterMap",
                "unionWith",
            ],
        ),
        (
            "Haskelujah.Set",
            &[
                "Set",
                "empty",
                "singleton",
                "fromList",
                "toList",
                "insert",
                "member",
                "delete",
                "size",
                "union",
                "intersection",
                "difference",
            ],
        ),
        (
            "Haskelujah.Pretty",
            &[
                "Doc", "text", "int", "nest", "line", "<+>", "$$", "hsep", "vsep", "indent",
                "render", "parens", "brackets", "braces",
            ],
        ),
        (
            "Haskelujah.Random",
            &[
                "Gen",
                "mkGen",
                "nextInt",
                "nextDouble",
                "randomList",
                "shuffle",
                "choice",
            ],
        ),
        ("Haskelujah.Process", &["shell", "exec", "ExitCode"]),
        ("Haskelujah.Time", &["now", "sleep", "measure"]),
        (
            "Haskelujah.Concurrent",
            &[
                "fork",
                "delay",
                "Chan",
                "newChan",
                "writeChan",
                "readChan",
                "MVar",
                "newMVar",
                "readMVar",
                "takeMVar",
                "putMVar",
            ],
        ),
        (
            "Haskelujah.Debug",
            &[
                "trace",
                "traceShow",
                "traceIO",
                "assert",
                "todo",
                "unreachable",
            ],
        ),
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in *exports_list_chirho {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
            if name_chirho.chars().next().is_some_and(|c| c.is_uppercase()) {
                let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
                exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            }
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
}
