// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: GHC's numeric, `Show`, `Read`, `Enum` and list modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use haskelujah_span_chirho::SpanChirho;

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, IfaceTypeChirho, ModuleIfaceChirho};

/// Push the interfaces for GHC's numeric, `Show`, `Read`, `Enum` and list modules.
pub(super) fn push_numeric_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // GHC.List
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map",
            "filter",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "foldr",
            "foldl",
            "foldl'",
            "scanl",
            "scanl'",
            "scanr",
            "iterate",
            "iterate'",
            "repeat",
            "replicate",
            "cycle",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "span",
            "break",
            "reverse",
            "and",
            "or",
            "any",
            "all",
            "elem",
            "notElem",
            "lookup",
            "zip",
            "zip3",
            "zipWith",
            "unzip",
            "unzip3",
            "concat",
            "concatMap",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.List".to_string(),
            exports_chirho,
        });
    }

    // GHC.Num
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "+",
            "-",
            "*",
            "negate",
            "abs",
            "signum",
            "fromInteger",
            "subtract",
            "integerToInt",
            "naturalToWord",
        ] {
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
        for name_chirho in &[
            "show",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
            "showList__",
        ] {
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
        for name_chirho in &[
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
            "minBound",
            "maxBound",
        ] {
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
        for name_chirho in &[
            "div",
            "mod",
            "quot",
            "rem",
            "divMod",
            "quotRem",
            "toInteger",
            "toRational",
            "fromIntegral",
            "realToFrac",
            "%",
            "numerator",
            "denominator",
            "reduce",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Integral",
            "Real",
            "RealFrac",
            "Fractional",
            "Ratio",
            "Rational",
        ] {
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
        for name_chirho in &[
            "pi",
            "exp",
            "log",
            "sqrt",
            "sin",
            "cos",
            "tan",
            "asin",
            "acos",
            "atan",
            "sinh",
            "cosh",
            "tanh",
            "float2Double",
            "double2Float",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
            "integerToFloat#",
            "integerToDouble#",
            "rationalToFloat",
            "rationalToDouble",
        ] {
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
        for name_chirho in &["byteSwap16", "byteSwap32", "byteSwap64"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Word8", "Word16", "Word32", "Word64"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Word".to_string(),
            exports_chirho,
        });
    }

    // Data.Char (already exists but GHC.Char doesn't)
    // GHC.Char
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["chr", "ord", "eqChar", "neChar", "unsafeChr"] {
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

    // Data.Char (extend existing with more functions)
    // Note: Data.Char already exists above, so this adds Data.Char.Internal
    // GHC.Unicode
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "isAlpha",
            "isAlphaNum",
            "isDigit",
            "isUpper",
            "isLower",
            "isSpace",
            "isPrint",
            "isControl",
            "isPunctuation",
            "toUpper",
            "toLower",
            "toTitle",
            "generalCategory",
            "GeneralCategory",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Unicode".to_string(),
            exports_chirho,
        });
    }

    // GHC.List (re-exports from GHC internal)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map",
            "filter",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "reverse",
            "foldl",
            "foldl'",
            "foldr",
            "foldr'",
            "scanl",
            "scanr",
            "iterate",
            "repeat",
            "replicate",
            "cycle",
            "take",
            "drop",
            "splitAt",
            "takeWhile",
            "dropWhile",
            "span",
            "break",
            "elem",
            "notElem",
            "lookup",
            "zip",
            "zip3",
            "zipWith",
            "zipWith3",
            "unzip",
            "unzip3",
            "lines",
            "words",
            "unlines",
            "unwords",
            "concat",
            "concatMap",
            "and",
            "or",
            "any",
            "all",
            "sum",
            "product",
            "maximum",
            "minimum",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.List".to_string(),
            exports_chirho,
        });
    }

    // GHC.Natural
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Natural", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "naturalToInteger",
            "naturalFromInteger",
            "naturalToWord",
            "wordToNatural",
            "intToNatural",
            "minusNaturalMaybe",
        ] {
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
        for name_chirho in &[
            "+",
            "-",
            "*",
            "negate",
            "abs",
            "signum",
            "fromInteger",
            "subtract",
        ] {
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
        for name_chirho in &["Integral", "Fractional", "Real", "RealFrac", "Rational"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Ratio", &[":%"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "toInteger",
            "toRational",
            "fromIntegral",
            "realToFrac",
            "quot",
            "rem",
            "div",
            "mod",
            "quotRem",
            "divMod",
            "(%)",
            "numerator",
            "denominator",
            "ceiling",
            "floor",
            "round",
            "truncate",
            "properFraction",
            "recip",
            "fromRational",
            ":%",
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
            "pi",
            "exp",
            "log",
            "sqrt",
            "sin",
            "cos",
            "tan",
            "asin",
            "acos",
            "atan",
            "sinh",
            "cosh",
            "tanh",
            "asinh",
            "acosh",
            "atanh",
            "(**)",
            "logBase",
            "floatRadix",
            "floatDigits",
            "floatRange",
            "decodeFloat",
            "encodeFloat",
            "exponent",
            "significand",
            "scaleFloat",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
            "atan2",
            "float2Double",
            "double2Float",
            "int2Double",
            "int2Float",
            "showFloat",
            "showEFloat",
            "showFFloat",
            "showGFloat",
            "castFloatToWord32",
            "castWord32ToFloat",
            "castDoubleToWord64",
            "castWord64ToDouble",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
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
        // Show class with methods for (..) import
        exports_chirho.types_chirho.insert(
            "Show".to_string(),
            IfaceTypeChirho {
                name_chirho: "Show".to_string(),
                constructors_chirho: vec![],
                methods_chirho: vec![
                    "showsPrec".to_string(),
                    "show".to_string(),
                    "showList".to_string(),
                ],
                is_class_chirho: true,
                span_chirho: SpanChirho::DUMMY_CHIRHO,
            },
        );
        for name_chirho in &[
            "show",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
            "showList__",
            "appPrec",
            "appPrec1",
            "showSpace",
            "showCommaSpace",
        ] {
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
        for name_chirho in &[
            "read",
            "reads",
            "readPrec",
            "readList",
            "readParen",
            "lex",
            "expectP",
            "list",
            "paren",
            "parens",
            "choose",
            "readListDefault",
            "readListPrecDefault",
            "readPrec_to_S",
            "readS_to_Prec",
            "readPrec_to_P",
            "readP_to_Prec",
            "readNumber",
            "readField",
            "readSymField",
            "reset",
            "step",
            "prec",
            "pfail",
            "get",
            "look",
            "readListPrec",
            "ReadPrec",
            "Lexeme",
        ] {
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
        for name_chirho in &[
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
            "minBound",
            "maxBound",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Enum".to_string(),
            exports_chirho,
        });
    }

    // GHC.Integer (compatibility)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Integer",
            "mkInteger",
            "smallInteger",
            "wordToInteger",
            "integerToWord",
            "integerToInt",
            "plusInteger",
            "timesInteger",
            "minusInteger",
            "negateInteger",
            "absInteger",
            "signumInteger",
            "divInteger",
            "modInteger",
            "quotInteger",
            "remInteger",
            "quotRemInteger",
            "divModInteger",
            "eqInteger",
            "neqInteger",
            "leInteger",
            "ltInteger",
            "geInteger",
            "gtInteger",
            "compareInteger",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        // Add integer logarithm primops
        for name_chirho in &[
            "integerLog2#",
            "integerLogBase#",
            "wordLog2#",
            "integerLog2",
            "integerLogBase",
            "wordLog2",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "GHC.Integer",
            "GHC.Integer.Logarithms",
            "GHC.Integer.Logarithms.Compat",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // GHC.Num.BigNat / GHC.Num.Integer (needed by hashable)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["BigNat", "bigNatToWord#", "bigNatToWord", "wordToBigNat#"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("BigNat", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num.BigNat".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["IS", "IN", "IP"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Integer", &["IS", "IN", "IP"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Num.Integer".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["IS", "IN", "IP"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Integer", &["IS", "IN", "IP"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.Integer.GMP.Internals".to_string(),
            exports_chirho,
        });
    }

    // Math.NumberTheory.Logarithms (integer-logarithms — needed by scientific)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "integerLog2'",
            "integerLog10'",
            "integerLogBase'",
            "integerLog2",
            "integerLog10",
            "integerLogBase",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Math.NumberTheory.Logarithms".to_string(),
            exports_chirho,
        });
    }
}
