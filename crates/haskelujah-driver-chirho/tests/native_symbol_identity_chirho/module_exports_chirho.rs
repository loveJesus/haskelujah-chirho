// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Row 621: `module M` in an export list exports every entity in scope both unqualified and as
//! `M.e` (Haskell 2010, 5.2). Each program and its expected output are GHC 9.14.1's, measured on
//! the text embedded here. The programs GHC runs are exact on the interpreter, LLVM and true
//! Cranelift; the ones GHC rejects are refused on both native backends. Row 621's p11 is row
//! 629's n1, byte for byte.
//! workflow: compiler-pipeline-chirho/value-scope-resolution-chirho

use super::import_identity_chirho::{
    ProgramChirho, assert_exact_chirho, assert_refused_natively_chirho,
};

/// p1: a self export takes an unqualified alias of another module. GHC prints '6'.
const P1_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module A) where
import B as A (myFun)
localA :: Int
localA = 5
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun localA)
"#,
        ),
    ],
    r#"6
"#,
);

/// p3: `module L` over an unqualified alias. GHC prints '2'.
const P3_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module L) where
import B as L (myFun)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun 1)
"#,
        ),
    ],
    r#"2
"#,
);

/// p4: the unqualified and `L.` scopes come from different imports. GHC prints '2'.
const P4_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module L) where
import qualified B as L (myFun)
import B (myFun)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun 1)
"#,
        ),
    ],
    r#"2
"#,
);

/// p5: `hiding` keeps the import's other names. GHC prints '7'.
const P5_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module L) where
import B as L hiding (myFun)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (other)
"#,
        ),
    ],
    r#"7
"#,
);

/// p7: a plain `module B` re-export. GHC prints '8'.
const P7_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module B) where
import B
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun other)
"#,
        ),
    ],
    r#"8
"#,
);

/// p9: a self export keeps the module's own definitions. GHC prints '5'.
const P9_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module A) where
import B as A (myFun)
localA :: Int
localA = 5
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (localA)
"#,
        ),
    ],
    r#"5
"#,
);

/// p10: one import's `hiding` keeps an explicitly exported local; it needs row 629's link. GHC prints '700'.
const P10_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (myFun, module L) where
import B as L hiding (myFun)
myFun :: Int -> Int
myFun = (* 100)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun other)
"#,
        ),
    ],
    r#"700
"#,
);

/// p2: a qualified-only alias exports nothing; GHC rejects it.
const P2_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module L) where
import qualified B as L (myFun)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun 1)
"#,
        ),
    ],
    r#"unbound variable: `myFun`"#,
);

/// p6: a hidden name is not re-exported; GHC rejects it.
const P6_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module L) where
import B as L hiding (myFun)
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (myFun 1)
"#,
        ),
    ],
    r#"unbound variable: `myFun`"#,
);

/// p8: a qualified self alias does not leak; GHC rejects it.
const P8_CHIRHO: (ProgramChirho, &str) = (
    &[
        (
            "B",
            r#"module B (myFun, other) where
myFun :: Int -> Int
myFun = (+ 1)
other :: Int
other = 7
"#,
        ),
        (
            "A",
            r#"module A (module A) where
import qualified B as A (other)
localA :: Int
localA = 5
"#,
        ),
        (
            "Main",
            r#"module Main where
import A
main :: IO ()
main = print (other + localA)
"#,
        ),
    ],
    r#"unbound variable: `other`"#,
);

#[test]
fn a_self_export_takes_an_unqualified_alias_of_another_module_chirho() {
    assert_exact_chirho(P1_CHIRHO);
}

#[test]
fn an_unqualified_alias_is_exported_chirho() {
    assert_exact_chirho(P3_CHIRHO);
}

#[test]
fn the_two_scopes_may_come_from_different_imports_chirho() {
    assert_exact_chirho(P4_CHIRHO);
}

#[test]
fn hiding_keeps_the_imports_other_names_chirho() {
    assert_exact_chirho(P5_CHIRHO);
}

#[test]
fn a_plain_module_re_export_exports_the_module_chirho() {
    assert_exact_chirho(P7_CHIRHO);
}

#[test]
fn a_self_export_keeps_the_modules_own_definitions_chirho() {
    assert_exact_chirho(P9_CHIRHO);
}

#[test]
fn a_hiding_import_keeps_an_explicitly_exported_local_chirho() {
    assert_exact_chirho(P10_CHIRHO);
}

/// The interpreter's multi-module path still runs all three, as it did on main:
/// `compile_modules_chirho` gives each module every upstream export's type by bare name, so
/// the unbound name type-checks and then links by name. That gap is outside row 621.
#[test]
fn names_out_of_both_scopes_are_not_exported_chirho() {
    assert_refused_natively_chirho(P2_CHIRHO);
    assert_refused_natively_chirho(P6_CHIRHO);
    assert_refused_natively_chirho(P8_CHIRHO);
}
