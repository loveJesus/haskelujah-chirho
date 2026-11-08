// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: numbers, bits, fixed point, and mutable references. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for numbers, bits, fixed point, and mutable references.
pub(super) fn push_numbers_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newIORef", "readIORef", "writeIORef", "modifyIORef"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.IORef".to_string(),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["byteSwap16", "byteSwap32", "byteSwap64"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Int".to_string(),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Ratio".to_string(),
            exports_chirho,
        });
    }

    // Data.Complex
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "realPart",
            "imagPart",
            "mkPolar",
            "cis",
            "polar",
            "magnitude",
            "phase",
            "conjugate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Complex", &[":+"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_val_chirho(":+");
        exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Complex".to_string(),
            exports_chirho,
        });
    }

    // GHC.Num.Natural / Numeric.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["naturalToWord", "naturalToInteger", "intToNatural"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base", "ghc-bignum"]),
            name_chirho: "GHC.Num.Natural".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho,
        });
    }

    // Data.Word / Data.Int
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
        let mut int_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Int", "Int8", "Int16", "Int32", "Int64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            int_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Int".to_string(),
            exports_chirho: int_exports_chirho,
        });
    }

    // Data.IORef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newIORef",
            "readIORef",
            "writeIORef",
            "modifyIORef",
            "modifyIORef'",
            "atomicModifyIORef",
            "atomicModifyIORef'",
            "atomicWriteIORef",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // Data.Version
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "showVersion",
            "parseVersion",
            "makeVersion",
            "versionBranch",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Version", &["Version"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Version".to_string(),
            exports_chirho,
        });
    }

    // Data.Bits
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            ".&.",
            ".|.",
            "xor",
            "complement",
            "shift",
            "shiftL",
            "shiftR",
            "rotate",
            "rotateL",
            "rotateR",
            "bit",
            "setBit",
            "clearBit",
            "complementBit",
            "testBit",
            "bitSizeMaybe",
            "bitSize",
            "isSigned",
            "popCount",
            "popCountDefault",
            "zeroBits",
            "finiteBitSize",
            "toIntegralSized",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        // Bits class with methods — (..) import brings these in
        exports_chirho.types_chirho.insert(
            "Bits".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "Bits".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    ".&.".to_string(),
                    ".|.".to_string(),
                    "xor".to_string(),
                    "complement".to_string(),
                    "shift".to_string(),
                    "shiftL".to_string(),
                    "shiftR".to_string(),
                    "rotate".to_string(),
                    "rotateL".to_string(),
                    "rotateR".to_string(),
                    "bit".to_string(),
                    "setBit".to_string(),
                    "clearBit".to_string(),
                    "complementBit".to_string(),
                    "testBit".to_string(),
                    "bitSizeMaybe".to_string(),
                    "bitSize".to_string(),
                    "isSigned".to_string(),
                    "popCount".to_string(),
                    "zeroBits".to_string(),
                    "unsafeShiftL".to_string(),
                    "unsafeShiftR".to_string(),
                ],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        exports_chirho.types_chirho.insert(
            "FiniteBits".to_string(),
            IfaceTypeChirho {
                declaration_chirho: None,
                name_chirho: "FiniteBits".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "finiteBitSize".to_string(),
                    "countLeadingZeros".to_string(),
                    "countTrailingZeros".to_string(),
                ],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Bits".to_string(),
            exports_chirho,
        });
    }

    // Numeric
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "showInt",
            "showHex",
            "showOct",
            "showFloat",
            "readInt",
            "readHex",
            "readOct",
            "readDec",
            "showSigned",
            "readSigned",
            "floatToDigits",
            "showIntAtBase",
            "readFloat",
            "showEFloat",
            "showFFloat",
            "showGFloat",
            "readBin",
            "Lexeme",
            "readLitChar",
            "showLitChar",
            "lexDigits",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Numeric".to_string(),
            exports_chirho,
        });
    }

    // Numeric.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho,
        });
    }

    // ── Data.Fixed ───────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Fixed",
            "HasResolution",
            "resolution",
            "Pico",
            "Nano",
            "Micro",
            "Milli",
            "Centi",
            "Deci",
            "Uni",
            "E0",
            "E1",
            "E2",
            "E3",
            "E6",
            "E9",
            "E12",
            "showFixed",
            "mod'",
            "div'",
            "divMod'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in [
            "Fixed",
            "HasResolution",
            "Pico",
            "Nano",
            "Micro",
            "Milli",
            "Centi",
            "Deci",
            "Uni",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Fixed".to_string(),
            exports_chirho,
        });
    }

    // ── Numeric / Numeric.Natural ────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "showInt",
            "showHex",
            "showOct",
            "readInt",
            "readHex",
            "readOct",
            "readDec",
            "readFloat",
            "readSigned",
            "Numeric",
            "fromRat",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Numeric".to_string(),
            exports_chirho,
        });

        let mut nat_exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_val_chirho("Natural");
        nat_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        nat_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Numeric.Natural".to_string(),
            exports_chirho: nat_exports_chirho,
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.IORef (some tests import this directly)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("IORef", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newIORef",
            "readIORef",
            "writeIORef",
            "modifyIORef",
            "modifyIORef'",
            "atomicModifyIORef",
            "atomicModifyIORef'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.IORef".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["byteSwap16", "byteSwap32", "byteSwap64"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Int".to_string(),
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
            ".&.",
            ".|.",
            "xor",
            "complement",
            "shift",
            "rotate",
            "setBit",
            "clearBit",
            "complementBit",
            "testBit",
            "bit",
            "zeroBits",
            "shiftL",
            "shiftR",
            "rotateL",
            "rotateR",
            "popCount",
            "popCountDefault",
            "bitSize",
            "bitSizeMaybe",
            "isSigned",
            "unsafeShiftL",
            "unsafeShiftR",
            "finiteBitSize",
            "countLeadingZeros",
            "countTrailingZeros",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Bits".to_string(),
            exports_chirho,
        });
    }

    // Data.Complex
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Complex", &[":+"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            ":+",
            "realPart",
            "imagPart",
            "mkPolar",
            "cis",
            "polar",
            "magnitude",
            "phase",
            "conjugate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
        for name_chirho in &["(%)", "numerator", "denominator", "approxRational", ":%"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Ratio".to_string(),
            exports_chirho,
        });
    }

    // Data.Fixed
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Fixed",
            "HasResolution",
            "E0",
            "E1",
            "E2",
            "E3",
            "E6",
            "E9",
            "E12",
            "Uni",
            "Deci",
            "Centi",
            "Milli",
            "Micro",
            "Nano",
            "Pico",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["resolution", "showFixed", "mod'", "divMod'", "div'"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
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
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Unique".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newSTRef",
            "readSTRef",
            "writeSTRef",
            "modifySTRef",
            "modifySTRef'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"STRef";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }

    // Data.Word
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["byteSwap16", "byteSwap32", "byteSwap64"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Word", "Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.Word".to_string(),
            exports_chirho,
        });
    }

    // Data.STRef
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "STRef",
            "newSTRef",
            "readSTRef",
            "writeSTRef",
            "modifySTRef",
            "modifySTRef'",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("STRef", &["STRef"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["base"]),
            name_chirho: "Data.STRef".to_string(),
            exports_chirho,
        });
    }
}
