// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: printing, reading, pretty-printing and parsers. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for printing, reading, pretty-printing and parsers.
pub(super) fn push_formats_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Text.Show
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "show",
            "showsPrec",
            "showString",
            "showChar",
            "showParen",
            "shows",
        ] {
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
        for name_chirho in &[
            "read",
            "reads",
            "readParen",
            "lex",
            "parens",
            "prec",
            "step",
            "reset",
            "readPrec",
            "readListPrec",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Read", "ReadS", "ReadPrec"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        if let Some(read_class_chirho) = exports_chirho.types_chirho.get_mut("Read") {
            read_class_chirho.methods_chirho = vec![
                "readsPrec".to_string(),
                "readList".to_string(),
                "readPrec".to_string(),
                "readListPrec".to_string(),
            ];
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Read".to_string(),
            exports_chirho,
        });
    }

    // ── Text.Printf ──────────────────────────────────────────────────
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "printf",
            "hPrintf",
            "PrintfArg",
            "HPrintfType",
            "PrintfType",
            "formatString",
            "formatInt",
            "formatFloat",
            "formatChar",
            "FormatParse",
            "FieldFormat",
            "FormatAdjustment",
            "FormatSign",
            "fpModifiers",
            "fpChar",
            "fpRest",
            "fmtWidth",
            "fmtPrecision",
            "fmtAdjust",
            "fmtSign",
            "fmtAlternate",
            "fmtModifiers",
            "fmtChar",
            "LeftAdjust",
            "ZeroPad",
            "SignPlus",
            "SignSpace",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["PrintfArg", "PrintfType", "HPrintfType"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, mut v_chirho) = mk_type_chirho("FormatParse", &["FormatParse"]);
        v_chirho.methods_chirho = vec![
            "fpModifiers".to_string(),
            "fpChar".to_string(),
            "fpRest".to_string(),
        ];
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, mut v_chirho) = mk_type_chirho("FieldFormat", &["FieldFormat"]);
        v_chirho.methods_chirho = vec![
            "fmtWidth".to_string(),
            "fmtPrecision".to_string(),
            "fmtAdjust".to_string(),
            "fmtSign".to_string(),
            "fmtAlternate".to_string(),
            "fmtModifiers".to_string(),
            "fmtChar".to_string(),
        ];
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for (type_name_chirho, constructors_chirho) in [
            ("FormatAdjustment", &["LeftAdjust", "ZeroPad"][..]),
            ("FormatSign", &["SignPlus", "SignSpace"][..]),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(type_name_chirho, constructors_chirho);
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
            "Read",
            "read",
            "reads",
            "readParen",
            "parens",
            "prec",
            "step",
            "reset",
            "readPrec",
            "readListPrec",
            "readMaybe",
            "readEither",
            "lex",
            "lexP",
            "lift",
            "choose",
            "look",
            "get",
            "pfail",
            "readS_to_Prec",
            "readPrec_to_S",
            "readP_to_S",
            "readS_to_P",
            "readPrec_to_P",
            "readP_to_Prec",
            "Lexeme",
            "ReadPrec",
            "ReadS",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in ["Read", "ReadPrec", "ReadS"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        if let Some(read_class_chirho) = exports_chirho.types_chirho.get_mut("Read") {
            read_class_chirho.methods_chirho = vec![
                "readsPrec".to_string(),
                "readList".to_string(),
                "readPrec".to_string(),
                "readListPrec".to_string(),
            ];
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Read".to_string(),
            exports_chirho: exports_chirho.clone(),
        });

        let mut show_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in [
            "Show",
            "show",
            "showsPrec",
            "showString",
            "showParen",
            "ShowS",
            "shows",
            "showChar",
            "showList",
            "showListWith",
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

    // Text.PrettyPrint
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Doc", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "text",
            "char",
            "int",
            "integer",
            "empty",
            "nest",
            "sep",
            "fsep",
            "hsep",
            "vcat",
            "hcat",
            "hang",
            "punctuate",
            "render",
            "parens",
            "brackets",
            "braces",
            "quotes",
            "doubleQuotes",
            "comma",
            "colon",
            "semi",
            "space",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.PrettyPrint".to_string(),
            exports_chirho,
        });
    }

    // Text.Parsec.Prim
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "parse",
            "runParser",
            "try",
            "lookAhead",
            "many",
            "skipMany",
            "unexpected",
            "parserFail",
            "getState",
            "putState",
            "modifyState",
            "getPosition",
            "setPosition",
            "tokenPrim",
            "tokenPrimEx",
            "tokens",
            "label",
            "(<?>)",
            "parserZero",
            "parserPlus",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Parsec", "ParsecT", "Stream"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Parsec.Prim".to_string(),
            exports_chirho,
        });
    }

    // Text.Parsec.Combinator
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "choice",
            "count",
            "between",
            "option",
            "optionMaybe",
            "optional",
            "many1",
            "skipMany1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "sepEndBy",
            "sepEndBy1",
            "chainl",
            "chainl1",
            "chainr",
            "chainr1",
            "eof",
            "notFollowedBy",
            "manyTill",
            "anyToken",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Parsec.Combinator".to_string(),
            exports_chirho,
        });
    }

    // Text.Parsec
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "parse",
            "parseTest",
            "try",
            "lookAhead",
            "many",
            "many1",
            "skipMany",
            "skipMany1",
            "choice",
            "option",
            "optional",
            "between",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainl1",
            "chainr1",
            "eof",
            "char",
            "string",
            "anyChar",
            "oneOf",
            "noneOf",
            "digit",
            "letter",
            "spaces",
            "newline",
            "tab",
            "upper",
            "lower",
            "alphaNum",
            "satisfy",
            "(<|>)",
            "(<?>)",
            "manyTill",
            "lookAhead",
            "notFollowedBy",
            "eof",
            "between",
            "option",
            "optional",
            "skipMany",
            "skipMany1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainl",
            "chainl1",
            "chainr",
            "chainr1",
            "count",
            "choice",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Parsec", "ParsecT", "SourcePos", "ParseError"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.Parsec".to_string(),
            exports_chirho,
        });
    }

    // Text.ParserCombinators.Parsec (re-export of parsec)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "parse",
            "parseTest",
            "try",
            "choice",
            "option",
            "many",
            "many1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainl",
            "chainl1",
            "chainr",
            "chainr1",
            "eof",
            "notFollowedBy",
            "lookAhead",
            "char",
            "string",
            "satisfy",
            "oneOf",
            "noneOf",
            "digit",
            "letter",
            "alphaNum",
            "space",
            "spaces",
            "anyChar",
            "newline",
            "tab",
            "upper",
            "lower",
            "(<?>)",
            "SourcePos",
            "sourceName",
            "sourceLine",
            "sourceColumn",
            "getPosition",
            "setPosition",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Parser", "ParseError", "SourcePos"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.ParserCombinators.Parsec".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.ParserCombinators.ReadP".to_string(),
            exports_chirho,
        });
    }

    // Prettyprinter (modern pretty-printing library)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Doc",
            "Pretty",
            "pretty",
            "viaShow",
            "unsafeViaShow",
            "emptyDoc",
            "nest",
            "line",
            "line'",
            "softline",
            "softline'",
            "hardline",
            "group",
            "flatAlt",
            "align",
            "hang",
            "indent",
            "encloseSep",
            "list",
            "tupled",
            "concatWith",
            "hsep",
            "vsep",
            "fillSep",
            "sep",
            "hcat",
            "vcat",
            "fillCat",
            "cat",
            "punctuate",
            "enclose",
            "surround",
            "annotate",
            "unAnnotate",
            "reAnnotate",
            "alterAnnotations",
            "angles",
            "braces",
            "brackets",
            "parens",
            "dquotes",
            "squotes",
            "lparen",
            "rparen",
            "lbrace",
            "rbrace",
            "lbracket",
            "rbracket",
            "langle",
            "rangle",
            "space",
            "dot",
            "equals",
            "pipe",
            "colon",
            "comma",
            "semi",
            "backslash",
            "slash",
            "layoutPretty",
            "layoutCompact",
            "layoutSmart",
            "defaultLayoutOptions",
            "LayoutOptions",
            "PageWidth",
            "SimpleDocStream",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Doc",
            "SimpleDocStream",
            "LayoutOptions",
            "PageWidth",
            "Pretty",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prettyprinter".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Prettyprinter.Internal".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        // Prettyprinter.Render.Terminal (ANSI terminal rendering)
        {
            let mut render_exports_chirho = exports_chirho.clone();
            for name_chirho in &[
                "renderStrict",
                "renderLazy",
                "renderIO",
                "putDoc",
                "hPutDoc",
                "AnsiStyle",
                "Color",
                "color",
                "colorDull",
                "bgColor",
                "bgColorDull",
                "bold",
                "italicized",
                "underlined",
            ] {
                let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
                render_exports_chirho
                    .values_chirho
                    .insert(k_chirho, v_chirho);
            }
            for name_chirho in &["AnsiStyle", "Color"] {
                let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
                render_exports_chirho
                    .types_chirho
                    .insert(k_chirho, v_chirho);
            }
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: "Prettyprinter.Render.Terminal".to_string(),
                exports_chirho: render_exports_chirho.clone(),
            });
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: "Prettyprinter.Render.Text".to_string(),
                exports_chirho: render_exports_chirho,
            });
        }
    }

    // Text.PrettyPrint.ANSI.Leijen (ansi-wl-pprint compat)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Doc",
            "Pretty",
            "pretty",
            "char",
            "text",
            "string",
            "int",
            "integer",
            "float",
            "double",
            "rational",
            "nest",
            "line",
            "linebreak",
            "group",
            "softline",
            "softbreak",
            "align",
            "hang",
            "indent",
            "encloseSep",
            "list",
            "tupled",
            "semiBraces",
            "hsep",
            "vsep",
            "fillSep",
            "sep",
            "hcat",
            "vcat",
            "fillCat",
            "cat",
            "punctuate",
            "enclose",
            "parens",
            "braces",
            "brackets",
            "angles",
            "squotes",
            "dquotes",
            "lparen",
            "rparen",
            "lbrace",
            "rbrace",
            "lbracket",
            "rbracket",
            "langle",
            "rangle",
            "space",
            "dot",
            "equals",
            "pipe",
            "colon",
            "comma",
            "semi",
            "backslash",
            "empty",
            "bold",
            "debold",
            "underline",
            "deunderline",
            "plain",
            "black",
            "red",
            "green",
            "yellow",
            "blue",
            "magenta",
            "cyan",
            "white",
            "dullblack",
            "dullred",
            "dullgreen",
            "dullyellow",
            "dullblue",
            "dullmagenta",
            "dullcyan",
            "dullwhite",
            "renderPretty",
            "renderCompact",
            "displayS",
            "displayIO",
            "SimpleDoc",
            "putDoc",
            "hPutDoc",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Doc", "SimpleDoc", "Pretty"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Text.PrettyPrint.ANSI.Leijen".to_string(),
            exports_chirho,
        });
    }

    // Text.ParserCombinators.ReadPrec / Text.ParserCombinators.ReadP
    for mod_name_chirho in &[
        "Text.ParserCombinators.ReadPrec",
        "Text.ParserCombinators.ReadP",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ReadPrec",
            "ReadP",
            "lift",
            "minPrec",
            "step",
            "reset",
            "prec",
            "readPrec_to_P",
            "readP_to_Prec",
            "readPrec_to_S",
            "readS_to_Prec",
            "get",
            "look",
            "pfail",
            "choice",
            "<++",
            "+++",
            "readS_to_P",
            "readP_to_S",
            "satisfy",
            "char",
            "string",
            "munch",
            "munch1",
            "skipSpaces",
            "between",
            "count",
            "option",
            "optional",
            "many",
            "many1",
            "skipMany",
            "skipMany1",
            "sepBy",
            "sepBy1",
            "endBy",
            "endBy1",
            "chainr",
            "chainl",
            "chainr1",
            "chainl1",
            "manyTill",
            "readParen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ReadPrec", "ReadP"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }
}
