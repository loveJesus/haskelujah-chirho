// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: generics, type-level literals, records, stacks and exceptions in `GHC.*`. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use haskelujah_ast_chirho::name_chirho::CompilerTypeConstructorChirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{
    mk_class_type_chirho, mk_data_alias_chirho, mk_data_type_chirho, mk_type_chirho, mk_val_chirho,
};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for generics, type-level literals, records, stacks and exceptions in `GHC.*`.
pub(super) fn push_generics_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal",
            "charVal",
            "symbolVal",
            "sameNat",
            "sameSymbol",
            "someNatVal",
            "someSymbolVal",
            "isDigit",
            "isSymbol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::NatChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_alias_chirho("Natural", CompilerTypeConstructorChirho::NatChirho);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::SymbolChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownCharChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownSymbolChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["SomeNat", "SomeSymbol", "TypeError", "ErrorMessage"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "from",
            "to",
            "from1",
            "to1",
            "datatypeName",
            "moduleName",
            "packageName",
            "selName",
            "conName",
            "conFixity",
            "conIsRecord",
            ":*:",
            ":+:",
            ":.:",
            "Comp1",
            "unComp1",
            "U1",
            "K1",
            "M1",
            "unK1",
            "unM1",
            "Par1",
            "unPar1",
            "Rec1",
            "unRec1",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Generic",
            "Generic1",
            "Rep",
            "Rep1",
            "V1",
            "U1",
            "K1",
            "M1",
            "Rec0",
            "Par1",
            "Rec1",
            "D1",
            "C1",
            "S1",
            "D",
            "C",
            "S",
            "R",
            ":*:",
            ":+:",
            ":.:",
            "Selector",
            "Constructor",
            "Datatype",
            "selName",
            "conName",
            "conFixity",
            "conIsRecord",
            "Fixity",
            "Prefix",
            "Infix",
            "Associativity",
            "LeftAssociative",
            "RightAssociative",
            "NotAssociative",
            "Meta",
            "MetaData",
            "MetaCons",
            "MetaSel",
            "SourceUnpackedness",
            "SourceStrictness",
            "DecidedStrictness",
            "NoSourceUnpackedness",
            "SourceNoUnpack",
            "SourceUnpack",
            "NoSourceStrictness",
            "SourceLazy",
            "SourceStrict",
            "DecidedLazy",
            "DecidedStrict",
            "DecidedUnpack",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for (type_name_chirho, constructors_chirho, methods_chirho) in [
            ("U1", vec!["U1"], vec![]),
            ("K1", vec!["K1"], vec!["unK1"]),
            ("M1", vec!["M1"], vec!["unM1"]),
            ("Par1", vec!["Par1"], vec!["unPar1"]),
            ("Rec1", vec!["Rec1"], vec!["unRec1"]),
            (":*:", vec![":*:"], vec![]),
            (":+:", vec!["L1", "R1"], vec![]),
            (":.:", vec!["Comp1"], vec!["unComp1"]),
        ] {
            exports_chirho.types_chirho.insert(
                type_name_chirho.to_string(),
                IfaceTypeChirho {
                    declaration_chirho: None,
                    name_chirho: type_name_chirho.to_string(),
                    constructors_chirho: constructors_chirho
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                    methods_chirho: methods_chirho.into_iter().map(str::to_string).collect(),
                    span_chirho: SpanChirho::DUMMY_CHIRHO,
                },
            );
        }
        for name_chirho in &["L1", "R1"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Generics".to_string(),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.StaticPtr".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal",
            "natVal'",
            "someNatVal",
            "sameNat",
            "cmpNat",
            "withSomeSNat",
            "withKnownNat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::NatChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "Natural", "SomeNat", "SNat", "CmpNat", "Div", "Mod", "Log2", "<=?",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callStack",
            "pushCallStack",
            "freezeCallStack",
            "emptyCallStack",
            "fromCallSiteList",
            "getCallStack",
            "srcLocFile",
            "srcLocModule",
            "srcLocStartLine",
            "srcLocEndLine",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("CallStack", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SrcLoc", &["SrcLoc"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Stack.Types".to_string(),
            exports_chirho,
        });
    }

    // GHC.Stack
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "callStack",
            "prettyCallStack",
            "getCallStack",
            "currentCallStack",
            "withFrozenCallStack",
            "freezeCallStack",
            "emptyCallStack",
            "pushCallStack",
            "srcLocFile",
            "srcLocModule",
            "srcLocPackage",
            "srcLocStartLine",
            "srcLocStartCol",
            "srcLocEndLine",
            "srcLocEndCol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["CallStack", "HasCallStack"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        {
            let (k_chirho, v_chirho) = mk_type_chirho(
                "SrcLoc",
                &[
                    "SrcLoc",
                    "srcLocFile",
                    "srcLocModule",
                    "srcLocPackage",
                    "srcLocStartLine",
                    "srcLocStartCol",
                    "srcLocEndLine",
                    "srcLocEndCol",
                ],
            );
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Err".to_string(),
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
        {
            let name_chirho = &"HasField";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.OverloadedLabels".to_string(),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Fingerprint".to_string(),
            exports_chirho,
        });
        // Also alias as GHC.Fingerprint.Type
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
            "SomeException",
            "Exception",
            "toException",
            "fromException",
            "displayException",
            "throw",
            "throwIO",
            "ErrorCall",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["SomeException", "Exception", "ErrorCall"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Exception".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Exception.Type".to_string(),
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
            "callStack",
            "getCallStack",
            "prettyCallStack",
            "prettySrcLoc",
            "currentCallStack",
            "withFrozenCallStack",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Stack".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::NatChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_alias_chirho("Natural", CompilerTypeConstructorChirho::NatChirho);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::SymbolChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownCharChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownSymbolChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["SomeNat", "SomeSymbol", "TypeError", "ErrorMessage"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "natVal",
            "natVal'",
            "charVal",
            "symbolVal",
            "symbolVal'",
            "someNatVal",
            "someSymbolVal",
            "sameNat",
            "sameSymbol",
            "isDigit",
            "isSymbol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats (re-exports from GHC.TypeLits plus extras)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::NatChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["SomeNat", "<=?"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["natVal", "natVal'", "someNatVal", "sameNat"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Generic",
            "Generic1",
            "Rep",
            "Rep1",
            "V1",
            "U1",
            "K1",
            "M1",
            "Par1",
            "Rec0",
            "Rec1",
            "D1",
            "C1",
            "S1",
            "Meta",
            "Datatype",
            "Constructor",
            "Selector",
            "Fixity",
            "Associativity",
            "SourceUnpackedness",
            "SourceStrictness",
            "DecidedStrictness",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "from",
            "to",
            "from1",
            "to1",
            "datatypeName",
            "moduleName",
            "packageName",
            "conName",
            "conFixity",
            "conIsRecord",
            "selName",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Generics".to_string(),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Records".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeLits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "natVal",
            "natVal'",
            "charVal",
            "symbolVal",
            "symbolVal'",
            "sameNat",
            "sameSymbol",
            "someNatVal",
            "someSymbolVal",
            "isDigit",
            "isSymbol",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::NatChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_alias_chirho("Natural", CompilerTypeConstructorChirho::NatChirho);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::SymbolChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownCharChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownSymbolChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "SomeNat",
            "SomeSymbol",
            "TypeError",
            "ErrorMessage",
            "AppendSymbol",
            "CmpNat",
            "CmpSymbol",
            "CharToNat",
            "NatToChar",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.TypeLits".to_string(),
            exports_chirho,
        });
    }

    // GHC.TypeNats
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["natVal", "natVal'", "sameNat", "someNatVal"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) =
            mk_data_type_chirho(CompilerTypeConstructorChirho::NatChirho, &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_class_type_chirho(
            haskelujah_ast_chirho::name_chirho::CompilerClassChirho::KnownNatChirho,
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["SomeNat", "CmpNat", "<=?"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.TypeNats".to_string(),
            exports_chirho,
        });
    }

    // GHC.Generics
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "from",
            "to",
            "from1",
            "to1",
            "datatypeName",
            "moduleName",
            "packageName",
            "conName",
            "conFixity",
            "conIsRecord",
            "selName",
            "selSourceUnpackedness",
            "selSourceStrictness",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Generic",
            "Generic1",
            "Rep",
            "Rep1",
            "V1",
            "U1",
            "K1",
            "M1",
            "Par1",
            "Rec1",
            "D1",
            "C1",
            "S1",
            "R",
            "D",
            "C",
            "S",
            "P",
            "Fixity",
            "Associativity",
            "SourceUnpackedness",
            "SourceStrictness",
            "DecidedStrictness",
            "Meta",
            "Datatype",
            "Constructor",
            "Selector",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "GHC.Generics".to_string(),
            exports_chirho,
        });
    }

    // GHC.SrcLoc
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "SrcLoc",
            "srcLocPackage",
            "srcLocModule",
            "srcLocFile",
            "srcLocStartLine",
            "srcLocStartCol",
            "srcLocEndLine",
            "srcLocEndCol",
            "getCallStack",
            "callStack",
            "withFrozenCallStack",
            "HasCallStack",
            "CallStack",
            "emptyCallStack",
            "freezeCallStack",
            "pushCallStack",
            "fromCallSiteList",
            "prettySrcLoc",
            "prettyCallStack",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["SrcLoc", "CallStack", "HasCallStack"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["SrcLoc"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "GHC.SrcLoc".to_string(),
            exports_chirho,
        });
    }
}
