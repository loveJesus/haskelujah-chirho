// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: `Prelude` and the list, character, `Maybe` and `Either` modules. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for `Prelude` and the list, character, `Maybe` and `Either` modules.
pub(super) fn push_prelude_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Data.List
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "map",
            "filter",
            "foldr",
            "foldl",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "reverse",
            "zip",
            "zipWith",
            "unzip",
            "take",
            "drop",
            "takeWhile",
            "dropWhile",
            "dropWhileEnd",
            "span",
            "break",
            "elem",
            "notElem",
            "lookup",
            "sum",
            "product",
            "minimum",
            "maximum",
            "sort",
            "insert",
            "nub",
            "concat",
            "concatMap",
            "any",
            "all",
            "iterate",
            "scanl",
            "partition",
            "sortBy",
            "insertBy",
            "nubBy",
            "maximumBy",
            "minimumBy",
            "foldl'",
            "foldl1'",
            "genericSplitAt",
            "genericLength",
            "genericTake",
            "genericDrop",
            "intercalate",
            "transpose",
            "subsequences",
            "permutations",
            "isPrefixOf",
            "isSuffixOf",
            "isInfixOf",
            "stripPrefix",
            "group",
            "groupBy",
            "inits",
            "tails",
            "unfoldr",
            "find",
            "elemIndex",
            "findIndex",
            "!!",
            "union",
            "\\\\",
            "lines",
            "unlines",
            "words",
            "unwords",
            "uncons",
            "singleton",
            "foldl1",
            "foldr1",
            "scanl1",
            "scanr",
            "scanr1",
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
            "ord",
            "chr",
            "isDigit",
            "isAlpha",
            "isAlphaNum",
            "isUpper",
            "isLower",
            "isSpace",
            "isLetter",
            "isPrint",
            "isControl",
            "isPunctuation",
            "isSeparator",
            "isAscii",
            "isLatin1",
            "isAsciiUpper",
            "isAsciiLower",
            "isHexDigit",
            "isOctDigit",
            "toLower",
            "toUpper",
            "toTitle",
            "digitToInt",
            "intToDigit",
            "showLitChar",
            "readLitChar",
            "lexLitChar",
            "generalCategory",
            "isSymbol",
            "isMark",
            "isNumber",
            "UppercaseLetter",
            "LowercaseLetter",
            "TitlecaseLetter",
            "ModifierLetter",
            "OtherLetter",
            "NonSpacingMark",
            "SpacingCombiningMark",
            "EnclosingMark",
            "DecimalNumber",
            "LetterNumber",
            "OtherNumber",
            "ConnectorPunctuation",
            "DashPunctuation",
            "OpenPunctuation",
            "ClosePunctuation",
            "InitialQuote",
            "FinalQuote",
            "OtherPunctuation",
            "MathSymbol",
            "CurrencySymbol",
            "ModifierSymbol",
            "OtherSymbol",
            "Space",
            "LineSeparator",
            "ParagraphSeparator",
            "Control",
            "Format",
            "Surrogate",
            "PrivateUse",
            "NotAssigned",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho(
            "GeneralCategory",
            &[
                "UppercaseLetter",
                "LowercaseLetter",
                "TitlecaseLetter",
                "ModifierLetter",
                "OtherLetter",
                "NonSpacingMark",
                "SpacingCombiningMark",
                "EnclosingMark",
                "DecimalNumber",
                "LetterNumber",
                "OtherNumber",
                "ConnectorPunctuation",
                "DashPunctuation",
                "OpenPunctuation",
                "ClosePunctuation",
                "InitialQuote",
                "FinalQuote",
                "OtherPunctuation",
                "MathSymbol",
                "CurrencySymbol",
                "ModifierSymbol",
                "OtherSymbol",
                "Space",
                "LineSeparator",
                "ParagraphSeparator",
                "Control",
                "Format",
                "Surrogate",
                "PrivateUse",
                "NotAssigned",
            ],
        );
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Char".to_string(),
            exports_chirho,
        });
    }

    // Data.Maybe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["maybe", "isJust", "isNothing", "fromMaybe", "fromJust"] {
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
            "IO",
            "String",
            "Char",
            "Int",
            "Integer",
            "Float",
            "Double",
            "Rational",
            "ShowS",
            "ReadS",
            "FilePath",
            "IOError",
            "Num",
            "Eq",
            "Ord",
            "Show",
            "Read",
            "Enum",
            "Bounded",
            "Functor",
            "Applicative",
            "Monad",
            "MonadFail",
            "Semigroup",
            "Monoid",
            "Foldable",
            "Traversable",
            "Coercible",
            "Const",
            "Word",
            "Integral",
            "Fractional",
            "Floating",
            "Real",
            "RealFrac",
            "RealFloat",
            "IOMode",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Standard Prelude functions
        for name_chirho in &[
            // Numeric
            "abs",
            "signum",
            "negate",
            "fromInteger",
            "fromIntegral",
            "toInteger",
            "even",
            "odd",
            "max",
            "min",
            "div",
            "mod",
            "quot",
            "rem",
            "divMod",
            "quotRem",
            "^",
            "^^",
            "succ",
            "pred",
            "toEnum",
            "fromEnum",
            "minBound",
            "maxBound",
            "floor",
            "ceiling",
            "round",
            "truncate",
            // Internal primops used by generated deriving methods.
            "==#",
            "eqFloat#",
            "eqStr#",
            // Boolean
            "not",
            "otherwise",
            // Tuple
            "fst",
            "snd",
            "curry",
            "uncurry",
            // Function
            "id",
            "const",
            "flip",
            "error",
            "undefined",
            "withFrozenCallStack",
            // List
            "map",
            "filter",
            "foldr",
            "foldl",
            "head",
            "tail",
            "last",
            "init",
            "null",
            "length",
            "reverse",
            "concat",
            "concatMap",
            "take",
            "drop",
            "takeWhile",
            "dropWhile",
            "span",
            "zip",
            "zipWith",
            "unzip",
            "!!",
            "elem",
            "notElem",
            "lookup",
            "sum",
            "product",
            "minimum",
            "maximum",
            "any",
            "all",
            "iterate",
            "scanl",
            "words",
            "unwords",
            "lines",
            "unlines",
            // IO
            "putStrLn",
            "putStr",
            "putChar",
            "print",
            "getLine",
            "getChar",
            "getContents",
            "interact",
            "readFile",
            "writeFile",
            "appendFile",
            // Show/Read
            "show",
            "read",
            // Monad/Functor/Foldable/Traversable
            "fmap",
            "return",
            "mapM_",
            "sequence_",
            "when",
            "unless",
            "foldMap",
            "traverse",
            // Conversion
            "fromString",
            "fromList",
            "toList",
            "bimap",
            // Comparison
            "compare",
            // String ops
            "intercalate",
            // Maybe/Either
            "maybe",
            "either",
            "fromMaybe",
            "isJust",
            "isNothing",
            // Data structures
            "sort",
            // Monad transformers
            "StateT",
            "runStateT",
            "evalStateT",
            "execStateT",
            "runState",
            "evalState",
            "execState",
            "get",
            "put",
            "modify",
            "bindStateT",
            "returnStateT",
            "MaybeT",
            "runMaybeT",
            "returnMaybeT",
            "bindMaybeT",
            // ReaderT
            "ReaderT",
            "runReaderT",
            "runReader",
            "ask",
            "local",
            "bindReaderT",
            "returnReaderT",
            // ExceptT
            "ExceptT",
            "runExceptT",
            "throwE",
            "returnExceptT",
            "bindExceptT",
            "catchE",
            // WriterT
            "WriterT",
            "runWriterT",
            "runWriter",
            "tell",
            "returnWriterT",
            "bindWriterT",
            "execWriterT",
            "execWriter",
            // NFData / deepseq
            "deepseq",
            "force",
            "evaluate",
            "$!!",
            // STM
            "newTVar",
            "readTVar",
            "writeTVar",
            "newTVarIO",
            "readTVarIO",
            "atomically",
            "retry",
            "orElse",
            // Data.Coerce
            "coerce",
            // Data.Function
            "on",
            "fix",
            // Numeric
            "realToFrac",
            // Missing standard Prelude functions
            "asTypeOf",
            "seq",
            "($)",
            "(.)",
            "(++)",
            "(&&)",
            "(||)",
            "(==)",
            "(/=)",
            "(<)",
            "(>)",
            "(<=)",
            "(>=)",
            "(+)",
            "(-)",
            "(*)",
            "(/)",
            "(**)",
            "logBase",
            "sqrt",
            "exp",
            "log",
            "sin",
            "cos",
            "tan",
            "asin",
            "acos",
            "atan",
            "atan2",
            "sinh",
            "cosh",
            "tanh",
            "asinh",
            "acosh",
            "atanh",
            "pi",
            "negate",
            "mapM",
            "sequence",
            "fail",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
            "readsPrec",
            "readParen",
            "reads",
            "lex",
            "break",
            "splitAt",
            "cycle",
            "repeat",
            "replicate",
            "and",
            "or",
            "enumFrom",
            "enumFromThen",
            "enumFromTo",
            "enumFromThenTo",
            "toRational",
            "fromRational",
            "recip",
            "divMod",
            "quotRem",
            "userError",
            "ioError",
            "until",
            "subtract",
            "gcd",
            "lcm",
            "rem",
            "(^)",
            "(^^)",
            "properFraction",
            "significand",
            "exponent",
            "floatRadix",
            "floatDigits",
            "decodeFloat",
            "encodeFloat",
            "scaleFloat",
            "isNaN",
            "isInfinite",
            "isDenormalized",
            "isNegativeZero",
            "isIEEE",
            "readLn",
            "readIO",
            "appendFile",
            "IOMode",
            "Word",
            "mempty",
            "mappend",
            "mconcat",
            "<>",
            "pure",
            "<*>",
            "*>",
            "<*",
            "Const",
            "getConst",
            "!!",
            "empty",
            "<|>",
            "some",
            "many",
            "optional",
            "liftA2",
            ">>",
            ">>=",
            "=<<",
            "foldr1",
            "foldl1",
            "foldl1'",
            "foldl'",
            "scanl1",
            "scanr",
            "scanr1",
            "catch",
            "throwIO",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude".to_string(),
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
        // base declares `data NonEmpty a = a :| [a]`: one constructor and no
        // field, exactly this entry, so `NonEmpty(..)` brings `:|` and nothing
        // else. workflow: compiler-pipeline-chirho/value-scope-resolution-chirho
        exports_chirho
            .complete_member_types_chirho
            .insert("NonEmpty".to_string());
        for con_name_chirho in &["(:|)", ":|"] {
            let (k_chirho, v_chirho) = mk_val_chirho(con_name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
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

    // Data.Ord
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["compare", "comparing", "clamp", "Down", "getDown"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Ord", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Ordering", &["LT", "EQ", "GT"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Down", &["Down"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Ord".to_string(),
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
        {
            let name_chirho = &"Solo";
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
        for name_chirho in &[
            "either",
            "lefts",
            "rights",
            "isLeft",
            "isRight",
            "fromLeft",
            "fromRight",
            "partitionEithers",
        ] {
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

    // Prelude.Experimental
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        // Re-export everything that Prelude exports (approximation)
        for name_chirho in &[
            "id",
            "const",
            "flip",
            "error",
            "undefined",
            "fmap",
            "pure",
            "return",
            "show",
            "print",
            "putStrLn",
            "putStr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prelude.Experimental".to_string(),
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

    // Data.Tuple.Solo
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Solo", &["MkSolo"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["getSolo", "MkSolo"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Tuple.Solo".to_string(),
            exports_chirho,
        });
    }

    // Data.Eq (base re-export)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Eq", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["==", "/="] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Data.Eq".to_string(),
            exports_chirho,
        });
    }
}
