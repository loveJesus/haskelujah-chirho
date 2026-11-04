// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `Typeable`, `Data`, type-level and coercion modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_data_type_chirho, mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};
use haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho;

/// Push the interfaces for `Typeable`, `Data`, type-level and coercion modules.
pub(super) fn push_reflection_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.Kind — Type, Constraint
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for constructor_chirho in &[
            CompilerTypeConstructorChirho::TypeUniverseChirho,
            CompilerTypeConstructorChirho::ConstraintUniverseChirho,
        ] {
            let (k_chirho, v_chirho) = mk_data_type_chirho(*constructor_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Kind".to_string(),
            exports_chirho,
        });
    }

    // Data.Proxy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::ProxyChirho, &["Proxy"]);
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
        for name_chirho in &[
            "castWith",
            "gcastWith",
            "testEquality",
            "sym",
            "trans",
            "inner",
            "outer",
        ] {
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
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "eqT",
            "gcast",
            "gcast1",
            "gcast2",
            "rnfTyCon",
            "rnfTypeRep",
        ] {
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

    // Type.Reflection
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "typeRepTyCon",
            "typeRepFingerprint",
            "someTypeRep",
            "rnfTypeRep",
            "rnfModule",
        ] {
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

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toConstr",
            "gunfold",
            "gfoldl",
            "dataTypeOf",
            "gmapT",
            "gmapQ",
            "gmapQl",
            "gmapQr",
            "gmapQi",
            "gmapM",
            "mkConstr",
            "mkDataType",
            "constrType",
            "showConstr",
            "cast",
            "gcast",
            "gcast1",
            "gcast2",
            "typeOf",
            "typeRep",
            "mkFunTy",
            "fromConstr",
            "fromConstrB",
            "fromConstrM",
            "dataTypeConstrs",
            "maxConstrIndex",
            "indexConstr",
            "dataTypeName",
            "constrFields",
            "constrFixity",
            "constrIndex",
            "constrRep",
            "repConstr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Data",
            "Typeable",
            "DataType",
            "Constr",
            "ConstrRep",
            "DataRep",
            "ConIndex",
            "Fixity",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "eqT",
            "gcast",
            "gcast1",
            "gcast2",
            "mkTyCon",
            "mkTyConApp",
            "typeRepTyCon",
            "typeRepArgs",
            "splitTyConApp",
            "funResultTy",
            "showsTypeRep",
            "rnfTyCon",
            "rnfTypeRep",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Typeable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("TypeRep", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::ProxyChirho, &["Proxy"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Data.CallStack (call-stack package, used by HUnit)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callStack",
            "HasCallStack",
            "withFrozenCallStack",
            "prettyCallStack",
            "getCallStack",
            "SrcLoc",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("CallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.CallStack".to_string(),
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

    // ── Type.Reflection ───────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "TypeRep",
            "SomeTypeRep",
            "Typeable",
            "typeRep",
            "typeRepFingerprint",
            "rnfTypeRep",
            "rnfModule",
            "eqTypeRep",
            "typeRepTyCon",
            "typeRepKind",
            "tyConPackage",
            "tyConModule",
            "tyConName",
            "tyConKindArgs",
            "tyConKindRep",
            "mkTyCon",
            "mkTrCon",
            "mkTrApp",
            "HRefl",
            "withTypeable",
            "pattern App",
            "pattern Con",
            "pattern Fun",
            "typeOf",
            "someTypeRep",
            "someTypeRepTyCon",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in [
            "TypeRep",
            "SomeTypeRep",
            "Typeable",
            "TyCon",
            "KindRep",
            "TypeLitSort",
            "Module",
            "Fingerprint",
            "(:~~:)",
        ] {
            let constructor_names_chirho: &[&str] = match name_chirho {
                "SomeTypeRep" => &["SomeTypeRep"],
                "(:~~:)" => &["HRefl"],
                _ => &[],
            };
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, constructor_names_chirho);
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

    // Data.Type.Equality (21 imports in GHC test suite)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho(":~:", &["Refl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("(:~~:)", &["HRefl"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, mut v_chirho) = mk_type_chirho("TestEquality", &[]);
        v_chirho.methods_chirho.push("testEquality".to_string());
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "Refl",
            "HRefl",
            "castWith",
            "gcastWith",
            "apply",
            "inner",
            "outer",
            "sym",
            "trans",
            "testEquality",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Type.Equality".to_string(),
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

    // Data.Proxy
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::ProxyChirho, &["Proxy"]);
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
            "typeOf",
            "typeRep",
            "cast",
            "gcast",
            "eqT",
            "typeRepTyCon",
            "mkTyCon3",
            "mkTyConApp",
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

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Data",
            "Typeable",
            "Constr",
            "DataType",
            "DataRep",
            "ConstrRep",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "toConstr",
            "gunfold",
            "gfoldl",
            "dataTypeOf",
            "mkConstr",
            "mkDataType",
            "constrIndex",
            "showConstr",
            "dataTypeConstrs",
            "dataTypeName",
            "constrType",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }

    // Data.Dynamic
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Dynamic", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "toDyn",
            "fromDyn",
            "fromDynamic",
            "dynTypeRep",
            "dynApply",
            "dynApp",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Dynamic".to_string(),
            exports_chirho,
        });
    }

    // Type.Reflection.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["mkTrApp", "mkTrCon", "typeRepFingerprint"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Type.Reflection.Unsafe".to_string(),
            exports_chirho,
        });
    }

    // Data.Coerce
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["coerce", "Coercible"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Coercible";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Coerce".to_string(),
            exports_chirho,
        });
    }

    // Data.Typeable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "typeOf",
            "typeRep",
            "cast",
            "gcast",
            "eqT",
            "typeRepTyCon",
            "typeRepFingerprint",
            "mkFunTy",
            "rnfTyCon",
            "rnfTypeRep",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Typeable", "TypeRep", "TyCon"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Typeable".to_string(),
            exports_chirho,
        });
    }

    // Data.Kind
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Type", "Constraint", "FUN"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Kind".to_string(),
            exports_chirho,
        });
    }

    // Data.Generics.Aliases (from syb)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "GenericT",
            "GenericQ",
            "GenericM",
            "GenericB",
            "GenericR",
            "GenericT'",
            "GenericQ'",
            "GenericM'",
            "mkT",
            "mkQ",
            "mkM",
            "mkMp",
            "mkR",
            "ext0",
            "extT",
            "extQ",
            "extM",
            "extMp",
            "extB",
            "extR",
            "orElse",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Generics.Aliases".to_string(),
            exports_chirho,
        });
    }

    // Data.Data
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Data",
            "DataType",
            "Constr",
            "ConstrRep",
            "IntConstr",
            "FloatConstr",
            "CharConstr",
            "mkDataType",
            "mkIntType",
            "mkFloatType",
            "mkCharType",
            "mkStringType",
            "mkConstr",
            "mkIntegralConstr",
            "mkRealConstr",
            "mkCharConstr",
            "mkNoRepType",
            "dataTypeName",
            "dataTypeConstrs",
            "dataTypeRep",
            "constrType",
            "constrRep",
            "constrFields",
            "constrFixity",
            "constrIndex",
            "showConstr",
            "readConstr",
            "isAlgType",
            "maxConstrIndex",
            "toConstr",
            "gunfold",
            "gfoldl",
            "gmapT",
            "gmapQ",
            "gmapQl",
            "gmapQr",
            "gmapQi",
            "gmapM",
            "gmapMp",
            "gmapMo",
            "dataTypeOf",
            "dataCast1",
            "dataCast2",
            "cast",
            "Typeable",
            "Fixity",
            "Prefix",
            "Infix",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Data",
            "DataType",
            "Constr",
            "ConstrRep",
            "Fixity",
            "Typeable",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Data".to_string(),
            exports_chirho,
        });
    }
}
