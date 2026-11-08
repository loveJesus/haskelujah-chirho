// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The builtin rows the dictionary pass writes for compiler classes, typed by compiler identity
//! (row 692, C10). `generate_builtin_prim_bindings_chirho` reads the table; each row is named
//! in its class's namespace, which is the legacy spelling unless a source class displaces it.
//! workflow: language-features-chirho/dictionary-evidence-chirho

use haskelujah_ast_chirho::name_chirho::CompilerClassChirho;

use crate::dict_chirho::DictPassCtxChirho;

impl DictPassCtxChirho {
    /// The builtin rows: (compiler class, method, type key, kind), where kind 0 is identity,
    /// 1 a binary primop and 2 a unary primop. The class column is the compiler identity; each
    /// row's legacy spelling derives from it, and its name from that class's namespace
    /// (row 692, C10).
    pub(in crate::dict_chirho) fn builtin_rows_chirho()
    -> Vec<(CompilerClassChirho, &'static str, &'static str, u8)> {
        use CompilerClassChirho::{
            EqChirho, FractionalChirho, IsListChirho, IsStringChirho, NumChirho, OrdChirho,
            ReadChirho, ShowChirho,
        };
        vec![
            // Eq
            (EqChirho, "==", "Int", 1),
            (EqChirho, "==", "Char", 1),
            (EqChirho, "==", "Bool", 1),
            // Ord
            (OrdChirho, "compare", "Int", 1),
            (OrdChirho, "compare", "Char", 1),
            (OrdChirho, "compare", "Double", 1),
            (OrdChirho, "compare", "Bool", 1),
            (OrdChirho, "compare", "[Char]", 1),
            // Show
            (ShowChirho, "show", "Int", 2),
            (ShowChirho, "show", "Char", 2),
            // Show Bool is handled specially below (case True/False -> string)
            (ShowChirho, "show", "Bool", 0),
            // Num
            (NumChirho, "+", "Int", 1),
            (NumChirho, "*", "Int", 1),
            (NumChirho, "-", "Int", 1),
            (NumChirho, "negate", "Int", 2),
            (NumChirho, "fromInteger", "Int", 0),
            // Double
            (EqChirho, "==", "Double", 1),
            (ShowChirho, "show", "Double", 2),
            (NumChirho, "+", "Double", 1),
            (NumChirho, "*", "Double", 1),
            (NumChirho, "-", "Double", 1),
            (NumChirho, "negate", "Double", 2),
            // fromInteger at Double is handled specially below: `fromIntegral# n`, typed
            // Int -> Double (`generate_from_integer_double_row_chirho`), never the identity.
            (NumChirho, "fromInteger", "Double", 0),
            // Fractional Double
            (FractionalChirho, "/", "Double", 1),
            (FractionalChirho, "recip", "Double", 2),
            (FractionalChirho, "fromRational", "Double", 0),
            // Read instances
            (ReadChirho, "read", "Int", 2),
            (ReadChirho, "read", "Double", 2),
            (ReadChirho, "read", "Bool", 2),
            // String (i.e. [Char])
            (EqChirho, "==", "[Char]", 1),
            (ShowChirho, "show", "[Char]", 2),
            // Lists
            (ShowChirho, "show", "[Int]", 2),
            // Maybe
            (ShowChirho, "show", "Maybe Int", 2),
            (ShowChirho, "show", "Maybe String", 2),
            (ShowChirho, "show", "Maybe Double", 2),
            // Tuples
            (ShowChirho, "show", "(Int,Int)", 2),
            (ShowChirho, "show", "(Int,String)", 2),
            (ShowChirho, "show", "(String,Int)", 2),
            (ShowChirho, "show", "(String,String)", 2),
            (ShowChirho, "show", "(Int,Bool)", 2),
            (ShowChirho, "show", "(Bool,Int)", 2),
            (ShowChirho, "show", "(Int,Double)", 2),
            (ShowChirho, "show", "(Double,Int)", 2),
            (ShowChirho, "show", "(Bool,Bool)", 2),
            (ShowChirho, "show", "(Double,Double)", 2),
            (ShowChirho, "show", "(Bool,String)", 2),
            (ShowChirho, "show", "(String,Bool)", 2),
            // 3-tuples
            (ShowChirho, "show", "(Int,Int,Int)", 2),
            (ShowChirho, "show", "(Int,Int,String)", 2),
            (ShowChirho, "show", "(String,Int,Int)", 2),
            (ShowChirho, "show", "(Int,String,Int)", 2),
            // Either
            (ShowChirho, "show", "Either Int Int", 2),
            (ShowChirho, "show", "Either String Int", 2),
            (ShowChirho, "show", "Either Int String", 2),
            (ShowChirho, "show", "Either String String", 2),
            // Ordering
            (ShowChirho, "show", "Ordering", 2),
            (EqChirho, "==", "Ordering", 1),
            (OrdChirho, "compare", "Ordering", 1),
            // IsString (OverloadedStrings)
            (IsStringChirho, "fromString", "[Char]", 0),
            // IsList (OverloadedLists) — identity for [a] (type key matches TyVarChirho(9037))
            (IsListChirho, "fromList", "[t9037]", 0),
            (IsListChirho, "toList", "[t9037]", 0),
        ]
    }

    /// The compiler classes the builtin table writes rows for.
    pub(in crate::dict_chirho) fn builtin_row_classes_chirho() -> Vec<CompilerClassChirho> {
        Self::builtin_rows_chirho()
            .into_iter()
            .map(|(identity_chirho, ..)| identity_chirho)
            .collect()
    }
}
